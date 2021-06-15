use std::env;
use std::sync::Arc;

use chrono::Duration;
use dotenv::dotenv;
use rocket::{http::Status, serde::json::Json, State};

use crate::github_api::GithubIssuesResponse;
use crate::{
    cache::Cache,
    github_api::good_github_issues,
    projects,
};

lazy_static! {
    static ref ISSUE_REGEN: Duration = {
        dotenv().unwrap();
        Duration::minutes(
            env::var("CACHE_TIME_ISSUES")
                .expect("No env var CACHE_TIME_ISSUES FOUND")
                .parse()
                .unwrap(),
        )
    };
}

#[get("/issues/<language>/<category>")]
pub async fn get_issues(
    language: String,
    category: usize,
    issues_cache: &State<Arc<Cache<(String, usize), GithubIssuesResponse>>>,
) -> Result<Json<GithubIssuesResponse>, Status> {
    if let Some(c) = issues_cache
        .get(&(language.clone(), category), *ISSUE_REGEN)
        .await
    {
        return Ok(Json(c));
    }

    let lang = projects::CONFIG.0.get(&language).ok_or(Status::NotFound)?;
    let group = lang.groups.get(category).ok_or(Status::NotFound)?;

    let issues = good_github_issues(&group.repos, &group.orgs, &group.flags).await?;
    issues_cache
        .insert((language, category), issues.clone())
        .await;

    Ok(Json(issues))
}
