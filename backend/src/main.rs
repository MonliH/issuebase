#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;

pub mod cache;
pub mod get_issues;
pub mod github_api;
pub mod projects;

use github_api::Issues;
use projects::{MinimalGroup, Projects};
use rocket::serde::json::Json;
use std::sync::Arc;

#[get("/projects")]
fn get_projects() -> Json<&'static Projects<MinimalGroup>> {
    Json(&*projects::PUBLIC_CONFIG)
}

const ISSUES_KEY: &'static str = "issues";

#[rocket::main]
async fn main() {
    let cache = Arc::new(
        cache::Cache::read_from_file(ISSUES_KEY)
            .unwrap_or_else(|| cache::Cache::<(String, usize), Vec<Issues>>::new()),
    );
    rocket::build()
        .manage(Arc::clone(&cache))
        .mount("/", routes![get_projects, get_issues::get_issues])
        .launch()
        .await
        .expect("Failed");

    Arc::try_unwrap(cache).unwrap().write_to_file(ISSUES_KEY).await;
}
