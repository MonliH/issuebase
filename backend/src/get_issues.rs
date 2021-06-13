use rocket::{http::Status, serde::json::Json};

use crate::{
    github_api::{good_github_issues, Issues},
    projects,
};

#[get("/issues/<language>/<category>")]
pub async fn get_issues(language: &str, category: usize) -> Result<Json<Vec<Issues>>, Status> {
    let language = projects::CONFIG.0.get(language).ok_or(Status::NotFound)?;
    let group = language.groups.get(category).ok_or(Status::NotFound)?;

    let issues = good_github_issues(&group.repos, &group.orgs, &group.flags).await?;

    Ok(Json(issues))
}
