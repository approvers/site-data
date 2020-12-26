use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Contacts {
    twitter: Option<u64>,
    github: Option<u64>,
    discord: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Member {
    id: String,
    name: String,
    contacts: Contacts,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    version: u32,
    members: Vec<Member>,
}

pub fn load(store_url: &str) -> Database {
    use std::fs::File;
    let mut store_file = File::create(store_url).expect("STOE_URL path must valid");
    let store: Database =
        serde_yaml::from_reader(&mut store_file).expect("the store must valid YAML");
    store
}
