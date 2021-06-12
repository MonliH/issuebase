use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env, fs::read_to_string};

lazy_static! {
    pub static ref CONFIG: Projects<Group> = {
        dotenv().ok();
        let projects =
            read_to_string(env::var("PROJECTS_PATH").expect("PROJECTS_PATH env var is not set"))
                .expect("failed to read projects path");
        toml::from_str(&projects).expect("invalid `Projects.toml` file")
    };
    pub static ref PUBLIC_CONFIG: Projects<MinimalGroup> = {
        Projects(
            CONFIG
                .0
                .clone()
                .into_iter()
                .map(|(key, value)| {
                    (
                        key,
                        Language {
                            groups: value
                                .groups
                                .into_iter()
                                .map(|g| MinimalGroup::from(g))
                                .collect::<Vec<MinimalGroup>>(),
                            id: value.id,
                            name: value.name,
                        },
                    )
                })
                .collect::<HashMap<_, Language<MinimalGroup>>>(),
        )
    };
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(transparent)]
pub struct Projects<T>(pub HashMap<String, Language<T>>);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Language<T> {
    pub name: String,
    pub id: String,
    pub groups: Vec<T>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Group {
    pub id: String,
    pub name: String,
    pub repos: Vec<String>,
    pub orgs: Vec<String>,
    pub flags: HashMap<String, Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MinimalGroup {
    pub id: String,
    pub name: String,
}

impl From<Group> for MinimalGroup {
    fn from(g: Group) -> Self {
        Self {
            id: g.id,
            name: g.name,
        }
    }
}
