use mongododm::{CollectionConfig, Index, IndexOption, Indexes, Model};
use serde::{Deserialize, Serialize};
use crate::user;
use user::User;
use mongowner::Schema;

pub struct PostCollConfig;

impl CollectionConfig for PostCollConfig {
    fn collection_name() -> &'static str {
        "post"
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Schema)]
pub struct Post {
    pub text: String,
    #[owned_by(User)]
    pub posted_by: u32,
    pub date: String,
}

impl Model for Post {
    type CollConf = PostCollConfig;
}
