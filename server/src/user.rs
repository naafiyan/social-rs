use mongododm::{CollectionConfig, Index, IndexOption, Indexes, Model};
use serde::{Deserialize, Serialize};

pub struct UserCollConfig;

impl CollectionConfig for UserCollConfig {
    fn collection_name() -> &'static str {
        "user"
    }

    // creating an index for test purposes
    fn indexes() -> Indexes {
        Indexes::new().with(Index::new("username").with_option(IndexOption::Unique))
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct User {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub age: u8,
    pub email: String,
}

impl Model for User {
    type CollConf = UserCollConfig;
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct UserB {
    pub username: u8,
    pub first_name: String,
    pub last_name: String,
}

impl Model for UserB {
    type CollConf = UserCollConfig;
}
