use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::env;
use std::mem;
use std::cmp::Reverse;

use chrono::{DateTime, Utc};
use reqwest::header::LINK;
use reqwest::header::{self, HeaderMap, HeaderValue};
use reqwest::Client;
use reqwest::RequestBuilder;
use rocket::http::Status;

use serde::{Deserialize, Serialize};

lazy_static! {
    pub static ref GITHUB_API_KEY: String = format!(
        "token {}",
        env::var("GITHUB_API_KEY").expect("GITHUB_API_KEY env var is not set")
    );
}

const API_URL: &str = "https://api.github.com/";

#[derive(Deserialize, Debug)]
struct GithubRepo {
    stargazers_count: usize,
    description: Option<String>,
    full_name: String,
    open_issues_count: usize,
}

#[derive(Deserialize, Debug)]
#[serde(transparent)]
struct GithubIssues(Vec<GithubIssue>);

#[derive(Deserialize, Debug)]
struct GithubIssue {
    html_url: String,
    title: String,
    created_at: DateTime<Utc>,
    labels: Vec<GithubLabel>,
    id: usize,
}

#[derive(Deserialize, Clone, Serialize, Debug)]
struct GithubLabel {
    name: String,
    color: String,
}

#[derive(Deserialize, Debug)]
struct Empty {}

async fn get_json<'a, T: for<'de> Deserialize<'de>>(req: RequestBuilder) -> Result<T, Status> {
    Ok(req
        .send()
        .await
        .map_err(|_| Status::InternalServerError)?
        .json()
        .await
        .map_err(|_| Status::InternalServerError)?)
}

async fn good_issues<'a>(
    client: &Client,
    repo: &str,
    repo_info: Option<GithubRepo>,
    flags: &[Cow<'a, String>],
) -> Result<Option<(Issues, usize)>, Status> {
    let info: GithubRepo = if let Some(info) = repo_info {
        info
    } else {
        get_json(client.get(format!("{}repos/{}", API_URL, repo))).await?
    };

    if info.open_issues_count == 0 {
        return Ok(None);
    }

    let mut issues = Vec::new();
    let mut isssue_ids = HashSet::new();
    for flag in flags {
        let mut url = format!("{}repos/{}/issues", API_URL, repo);
        loop {
            let res = client
                .get(mem::take(&mut url))
                .query(&[("sort", "created"), ("labels", flag), ("per_page", "100"), ("assignee", "none")]).send().await.map_err(|_| Status::InternalServerError)?;
            let mut link = if let Some(link) = res.headers().get(LINK) {
                Some(parse_link_header::parse(
                                link.to_str()
                    .unwrap())
                .unwrap())
            } else {None};

            let mut is: GithubIssues = res.json().await.map_err(|_| Status::InternalServerError)?;
            issues.extend(is.0.iter_mut().filter(|i| !isssue_ids.contains(&i.id)).map(|i| Issue {
                title: mem::take(&mut i.title),
                url: mem::take(&mut i.html_url),
                date: i.created_at,
                labels: mem::take(&mut i.labels),
            }));
            isssue_ids.extend(is.0.iter().map(|i| i.id));

            if let Some(Some(u)) = link.as_mut().map(|l| l.get_mut(&Some("next".to_string()))) {
                url = mem::take(&mut u.raw_uri);
            } else {break}
        }
    }

    if issues.is_empty() {
        return Ok(None);
    }

    Ok(Some((Issues {
        issues,
        repo_name: repo.to_string(),
        stars: info.stargazers_count,
        description: info.description,
    }, info.open_issues_count)))
}

async fn get_org_repos(client: &Client, org: &str) -> Result<Vec<GithubRepo>, Status> {
    let res = client
        .get(format!("{}orgs/{}/repos", API_URL, org))
        .query(&[("per_page", 100)])
        .send()
        .await
        .map_err(|_| Status::InternalServerError)?;
    let mut link = if let Some(link) = res.headers().get(LINK) {
        Some(parse_link_header::parse(
                        link.to_str()
            .unwrap())
        .unwrap())
    } else {None};

    let mut repos: Vec<GithubRepo> = res.json().await.map_err(|_| Status::InternalServerError)?;
    repos = repos
        .into_iter()
        .filter(|r| r.open_issues_count != 0)
        .collect();
    while let Some(Some(url)) = link.as_mut().map(|l| l.get_mut(&Some("next".to_string()))) {
        let res = client
            .get(mem::take(&mut url.raw_uri))
            .send()
            .await
            .map_err(|_| Status::InternalServerError)?;
        if let Some(l) = res.headers().get(LINK) {
            link = Some(parse_link_header::parse(l.to_str().unwrap()).unwrap());
        }
        let more_repos: Vec<GithubRepo> =
            res.json().await.map_err(|_| Status::InternalServerError)?;
        repos.extend(more_repos.into_iter().filter(|r| r.open_issues_count != 0));
    }
    Ok(repos)
}

pub async fn good_github_issues(
    repos: &[String],
    orgs: &[String],
    flags: &HashMap<String, Vec<String>>,
) -> Result<GithubIssuesResponse, Status> {
    let mut headers = HeaderMap::new();
    let mut auth_value = HeaderValue::from_static(&*GITHUB_API_KEY);
    auth_value.set_sensitive(true);
    headers.insert(header::AUTHORIZATION, auth_value);

    let client = reqwest::Client::builder()
        .user_agent("issuebase")
        .default_headers(headers)
        .build()
        .unwrap();

    // Allocate a single buffer used for flags in each repo
    let mut flags_buf: Vec<Cow<String>> = Vec::new();

    let mut all_repos: Vec<(Cow<String>, Option<GithubRepo>, Option<&str>)> = Vec::new();
    all_repos.extend(repos.iter().map(|r| (Cow::Borrowed(r), None, None)));
    for org in orgs {
        for mut repo in get_org_repos(&client, org).await? {
            all_repos.push((Cow::Owned(mem::take(&mut repo.full_name)), Some(repo), Some(org)));
        }
    }

    let mut all_issues = Vec::new();
    let mut issues_scanned = 0;
    let mut issues_found = 0;

    for (repo, prev_info, org) in all_repos {
        if let Some(repo_flags) = flags.get(repo.as_ref()) {
            flags_buf.extend(repo_flags.iter().map(|s| Cow::Borrowed(s)));
        } else {
            // "*" marks a flag used in all issues in this group.
            // Usually set to "good first issue".
            if let Some(initial) = flags.get("*") {
                flags_buf.extend(initial.iter().map(|s| Cow::Borrowed(s)));
            }
        }

        if let Some(Some(org_flags)) = org.map(|o| flags.get(o)) {
            flags_buf.extend(org_flags.iter().map(|s| Cow::Borrowed(s)));
        }

        if let Some((issues, total)) = good_issues(&client, &repo, prev_info, &flags_buf).await? {
            issues_found += issues.issues.len();
            all_issues.push(issues);
            issues_scanned += total;
        }

        // Resuse the allocated buffer by removing the added elements
        // this is practically free: it's just a pointer decrement
        flags_buf.clear()
    }

    // sort repos by most stars
    all_issues.sort_by_key(|proj| Reverse(proj.stars));

    Ok(GithubIssuesResponse { issues: all_issues, issues_scanned, issues_found })
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Issues {
    issues: Vec<Issue>,
    repo_name: String,
    stars: usize,
    description: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Issue {
    title: String,
    url: String,
    date: chrono::DateTime<Utc>,
    labels: Vec<GithubLabel>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GithubIssuesResponse {
    issues: Vec<Issues>,
    issues_scanned: usize,
    issues_found: usize,
}
