use rocket::{http::Status, serde::json::Json};
use serde::Serialize;

use crate::{github_api::good_github_issues, projects};

#[get("/issues/<language>/<category>")]
pub async fn get_issues(language: &str, category: usize) -> Result<Json<IssuesSchema>, Status> {
    let language = projects::CONFIG.0.get(language).ok_or(Status::NotFound)?;
    let group = language.groups.get(category).ok_or(Status::NotFound)?;

    good_github_issues(&group.repos, &group.orgs, &group.flags).await?;

    Ok(Json(IssuesSchema {}))
}

#[derive(Serialize)]
pub struct IssuesSchema {}
