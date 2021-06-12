use std::borrow::Cow;
use std::collections::HashMap;

use chrono::NaiveDateTime;
use reqwest::Client;
use rocket::http::Status;

use serde::{Serialize, Deserialize};

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
    created_at: NaiveDateTime,
    assignees: Option<Empty>
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
        println!("{:?}", res);
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
    println!("{:?} {:?}", repos, orgs);
    let client = reqwest::Client::builder()
        .user_agent("devcontrib")
        .build().unwrap();
    // Allocate a single buffer used for flags in each repo
    let mut flags_buf: Vec<Cow<String>> = Vec::new();

    // "*" marks a flag used in all issues in this group.
    // Usually set to "good first issue".
    if let Some(initial) = flags.get("*") {
        flags_buf.extend(initial.iter().map(|s| Cow::Borrowed(s)));
    }

    let mut all_repos = Vec::new();
    all_repos.extend(repos);

    let mut all_issues = Vec::new();

    for repo in all_repos {
        let flags_len = if let Some(repo_flags) = flags.get(repo) {
            flags_buf.extend(repo_flags.iter().map(|s| Cow::Borrowed(s)));
            repo_flags.len()
        } else {
            0
        };
        println!("{:?}", repo);

        all_issues.push(good_issues(&client, repo, &flags_buf).await?);

        // Resuse the allocated buffer by removing the added elements
        // this is practically free: it's just a pointer decrement
        flags_buf.truncate(flags_buf.len().saturating_sub(flags_len))
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
    date: chrono::NaiveDateTime,
    flags: Vec<String>,
}
