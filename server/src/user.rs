use mongododm::{CollectionConfig, Index, IndexOption, Indexes, Model};
use mongododm::mongo::bson::oid::ObjectId;
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
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub age: u8,
    pub email: String,
}

impl Model for User {
    type CollConf = UserCollConfig;
}
