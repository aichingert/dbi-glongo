use serde::{Deserialize, Serialize};

use crate::database::entities::Content;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntryDto {
    pub title: String,
    pub author: AuthorDto,
    pub description: String,

    pub creation_date: bson::DateTime,
    pub edit_dates: Vec<bson::DateTime>,

    pub impression_count: u64,

    pub content: Content,

    pub comments_allowed: bool,
    pub categories: Vec<String>,
    pub comments: Vec<CommentDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorDto {
    #[serde(rename(deserialize = "_id"))]
    pub id: Option<bson::oid::ObjectId>,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentDto {
    pub text: String,
    pub author: AuthorDto,
    pub creation_date: bson::DateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntryApiDto {
    pub title: String,
    pub description: String,
    pub text: String,

    pub comments_allowed: Option<String>,
    pub author: String,
    pub image: Option<String>,
}
