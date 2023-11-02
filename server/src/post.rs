use mongododm::{CollectionConfig, Index, IndexOption, Indexes, Model};
use serde::{Deserialize, Serialize};
use crate::user;
use user::User;
use mongowner::Schema;
use mongododm::delete::SafeDeleteable;
use mongododm::delete::Schemable;
use mongododm::mongo::bson::oid::ObjectId;

pub struct PostCollConfig;

impl CollectionConfig for PostCollConfig {
    fn collection_name() -> &'static str {
        "post"
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Schema)]
#[collection(posts)]
pub struct Post {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub text: String,
    #[owned_by(User)]
    pub posted_by: u32,
    pub date: String,
}

impl Model for Post {
    type CollConf = PostCollConfig;
}
