use mongododm::{CollectionConfig, Index, IndexOption, Indexes, Model};
use mongowner::owned_by;
use serde::{Deserialize, Serialize};
use user_schema::User;

use crate::user_schema;

pub struct PostCollConfig;

impl CollectionConfig for PostCollConfig {
    fn collection_name() -> &'static str {
        "post"
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[owned_by(User)]
pub struct Post {
    pub text: String,
    pub posted_by: User,
    pub date: String,
}

impl Model for Post {
    type CollConf = PostCollConfig;
}
