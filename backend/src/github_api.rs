use std::borrow::Cow;
use std::collections::HashMap;
use std::env;

use chrono::{Utc, DateTime};
use reqwest::Client;
use reqwest::header::{self, HeaderValue, HeaderMap};
use rocket::http::Status;

use serde::{Serialize, Deserialize};

lazy_static! {
    pub static ref GITHUB_API_KEY: String =
        format!("token {}", env::var("GITHUB_API_KEY").expect("GITHUB_API_KEY env var is not set"));
}

const API_URL: &str = "https://api.github.com/";

#[derive(Deserialize, Debug)]
struct GithubRepo {
    stargazers_count: usize
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
    labels: Vec<GithubLabel>
}

#[derive(Deserialize, Serialize, Debug)]
struct GithubLabel {
    name: String,
    color: String,
}

#[derive(Deserialize, Debug)]
struct Empty {}

async fn good_issues<'a>(client: &Client, repo: &str, flags: &[Cow<'a, String>]) -> Result<Issues, Status> {
    let repo_info: GithubRepo = client
        .get(format!("{}repos/{}", API_URL, repo))
        .send()
        .await
        .map_err(|_| Status::InternalServerError)?
        .json()
        .await
        .map_err(|e| Status::InternalServerError)?;
    let mut issues = Vec::new();
    for flag in flags {
        let res: GithubIssues = client
            .get(format!("{}repos/{}/issues", API_URL, repo))
            .query(&[("sort", "created"), ("labels", flag)])
            .send()
            .await
            .map_err(|e| {dbg!(e); Status::InternalServerError})?
            .json()
            .await
            .map_err(|e| {dbg!(e); Status::InternalServerError})?;
        issues.extend(
            res
                .0
                .into_iter()
                .map(|i| Issue {
                    title: i.title,
                    url: i.html_url,
                    date: i.created_at,
                    labels: i.labels
                })
        );
    }
    Ok(Issues {
        issues,
        repo_name: repo.to_string(),
        stars: repo_info.stargazers_count,
    })
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
        .build().unwrap();
    // Allocate a single buffer used for flags in each repo
    let mut flags_buf: Vec<Cow<String>> = Vec::new();


    let mut all_repos = Vec::new();
    all_repos.extend(repos);

    let mut all_issues = Vec::new();

    for repo in all_repos {
        if let Some(repo_flags) = flags.get(repo) {
            flags_buf.extend(repo_flags.iter().map(|s| Cow::Borrowed(s)));
        } else {
            // "*" marks a flag used in all issues in this group.
            // Usually set to "good first issue".
            if let Some(initial) = flags.get("*") {
                flags_buf.extend(initial.iter().map(|s| Cow::Borrowed(s)));
            }
        }

        all_issues.push(good_issues(&client, repo, &flags_buf).await?);

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
}

#[derive(Debug, Serialize)]
pub struct Issue {
    title: String,
    url: String,
    date: chrono::DateTime<Utc>,
    labels: Vec<GithubLabel>,
}
