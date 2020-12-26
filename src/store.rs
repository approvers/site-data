use serde::{Deserialize, Serialize};

#[readonly::make]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contacts {
    pub twitter: Option<u64>,
    pub github: Option<u64>,
    pub discord: Option<u64>,
}

#[readonly::make]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Member {
    pub id: String,
    pub name: String,
    pub contacts: Contacts,
}

#[readonly::make]
#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    version: u32,
    pub members: Vec<Member>,
}

pub fn load(store_url: &str) -> Database {
    use std::fs::File;
    let mut store_file = File::create(store_url).expect("STOE_URL path must valid");
    let store: Database =
        serde_yaml::from_reader(&mut store_file).expect("the store must valid YAML");
    assert_eq!(store.version, 3);
    store
}
