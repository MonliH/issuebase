#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;

pub mod get_issues;
pub mod github_api;
pub mod projects;

use projects::{MinimalGroup, Projects};
use rocket::{http::Status, serde::json::Json};

#[get("/projects")]
fn get_projects() -> Json<&'static Projects<MinimalGroup>> {
    Json(&*projects::PUBLIC_CONFIG)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_projects, get_issues::get_issues])
}
