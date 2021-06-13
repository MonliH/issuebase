use std::borrow::Cow;
use std::collections::HashMap;
use std::env;
use std::mem;

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
    assignee: Option<Empty>,
    labels: Vec<GithubLabel>,
}

#[derive(Deserialize, Serialize, Debug)]
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
) -> Result<Option<Issues>, Status> {
    let info: GithubRepo = if let Some(info) = repo_info {
        info
    } else {
        get_json(client.get(format!("{}repos/{}", API_URL, repo))).await?
    };

    if info.open_issues_count == 0 {
        return Ok(None);
    }

    let mut issues = Vec::new();
    for flag in flags {
        let res: GithubIssues = get_json(
            client
                .get(format!("{}repos/{}/issues", API_URL, repo))
                .query(&[("sort", "created"), ("labels", flag)]),
        )
        .await?;
        issues.extend(res.0.into_iter().map(|i| Issue {
            title: i.title,
            url: i.html_url,
            date: i.created_at,
            labels: i.labels,
        }));
    }

    Ok(Some(Issues {
        issues,
        repo_name: repo.to_string(),
        stars: info.stargazers_count,
        description: info.description,
    }))
}

async fn get_org_repos(client: &Client, org: &str) -> Result<Vec<GithubRepo>, Status> {
    let res = client
        .get(format!("{}orgs/{}/repos", API_URL, org))
        .query(&[("per_page", 100)])
        .send()
        .await
        .map_err(|_| Status::InternalServerError)?;
    let mut link =
        parse_link_header::parse(res.headers().get(LINK).unwrap().to_str().unwrap()).unwrap();
    let mut repos: Vec<GithubRepo> = res.json().await.map_err(|_| Status::InternalServerError)?;
    repos = repos
        .into_iter()
        .filter(|r| r.open_issues_count != 0)
        .collect();
    while let Some(url) = link.get_mut(&Some("next".to_string())) {
        let res = client
            .get(mem::take(&mut url.raw_uri))
            .send()
            .await
            .map_err(|_| Status::InternalServerError)?;
        link =
            parse_link_header::parse(res.headers().get(LINK).unwrap().to_str().unwrap()).unwrap();
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
) -> Result<Vec<Issues>, Status> {
    let mut headers = HeaderMap::new();
    let mut auth_value = HeaderValue::from_static(&*GITHUB_API_KEY);
    auth_value.set_sensitive(true);
    headers.insert(header::AUTHORIZATION, auth_value);

    let client = reqwest::Client::builder()
        .user_agent("devcontrib")
        .default_headers(headers)
        .build()
        .unwrap();

    // Allocate a single buffer used for flags in each repo
    let mut flags_buf: Vec<Cow<String>> = Vec::new();

    let mut all_repos: Vec<(Cow<String>, Option<GithubRepo>)> = Vec::new();
    all_repos.extend(repos.iter().map(|r| (Cow::Borrowed(r), None)));
    for org in orgs {
        for mut repo in get_org_repos(&client, org).await? {
            all_repos.push((Cow::Owned(mem::take(&mut repo.full_name)), Some(repo)));
        }
    }

    let mut all_issues = Vec::new();

    for (repo, prev_info) in all_repos {
        if let Some(repo_flags) = flags.get(repo.as_ref()) {
            flags_buf.extend(repo_flags.iter().map(|s| Cow::Borrowed(s)));
        } else {
            // "*" marks a flag used in all issues in this group.
            // Usually set to "good first issue".
            if let Some(initial) = flags.get("*") {
                flags_buf.extend(initial.iter().map(|s| Cow::Borrowed(s)));
            }
        }

        if let Some(issues) = good_issues(&client, &repo, prev_info, &flags_buf).await? {
            all_issues.push(issues);
        }

        // Resuse the allocated buffer by removing the added elements
        // this is practically free: it's just a pointer decrement
        flags_buf.clear()
    }

    Ok(all_issues)
}

#[derive(Debug, Serialize)]
pub struct Issues {
    issues: Vec<Issue>,
    repo_name: String,
    stars: usize,
    description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Issue {
    title: String,
    url: String,
    date: chrono::DateTime<Utc>,
    labels: Vec<GithubLabel>,
}
