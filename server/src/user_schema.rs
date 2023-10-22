use mongodm::{sync_indexes, CollectionConfig, Index, IndexOption, Indexes, Model, ToRepository};
use serde::{Deserialize, Serialize};

struct UserCollConfig;

impl CollectionConfig for UserCollConfig {
    fn collection_name() -> &'static str {
        "user"
    }

    // creating an index for test purposes
    fn indexes() -> Indexes {
        Indexes::new().with(Index::new("username").with_option(IndexOption::Unique))
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct User {
    username: String,
    first_name: String,
    last_name: String,
    age: u8,
    email: String,
}

impl Model for User {
    type CollConf = UserCollConfig;
}
