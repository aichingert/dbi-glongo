use leptos::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PostError {
    #[error("Invalid post ID.")]
    InvalidArticle,
    #[error("Post not found.")]
    PostNotFound,
    #[error("Server error.")]
    ServerError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Entry {
    title: String,
    author: bson::oid::ObjectId,
    description: String,

    creation_date: bson::DateTime,
    edit_dates: Vec<bson::DateTime>,
    impression_count: u64,

    content: Content,

    comments_allowed: bool,
    categories: Vec<String>,
    comments: Vec<Comment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    pub text: String,
    pub links: Vec<String>,
    pub coordinates: Vec<Coordinate>,
    pub images: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Coordinate {
    long: f64,
    lat: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Comment {
    text: String,
    creation_date: bson::DateTime,
    author: bson::oid::ObjectId,
}

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

impl EntryDto {
    fn from_entry_with_authors(entry: Entry, authors: &Vec<AuthorDto>) -> Self {
        let author = authors
            .iter()
            .find(|&author| author.id == entry.author)
            .unwrap()
            .clone();

        let mut comments = Vec::new();

        for comment in entry.comments.iter() {
            comments.push(CommentDto {
                text: comment.text.clone(),
                author: authors
                    .iter()
                    .find(|&author| author.id == comment.author)
                    .unwrap()
                    .clone(),
                creation_date: comment.creation_date,
            });
        }

        Self {
            title: entry.title,
            author,
            description: entry.description,

            creation_date: entry.creation_date,
            edit_dates: entry.edit_dates,

            impression_count: entry.impression_count,

            content: entry.content,

            comments_allowed: entry.comments_allowed,
            categories: entry.categories,
            comments,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorDto {
    #[serde(rename(deserialize = "_id"))]
    id: bson::oid::ObjectId,
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

#[server]
pub async fn get_all_entries() -> Result<Vec<EntryDto>, ServerFnError> {
    use futures_util::StreamExt;
    use mongodb::{bson::doc, options::FindOptions};

    let client =
        mongodb::Client::with_uri_str("mongodb://root:root@localhost/db?authSource=admin").await?;
    let database = client.database("blogDB");

    let cursor = database
        .collection::<AuthorDto>("users")
        .find(None, None)
        .await?;
    let authors = cursor
        .collect::<Vec<Result<AuthorDto, _>>>()
        .await
        .into_iter()
        .flatten()
        .collect::<Vec<AuthorDto>>();

    let cursor = database
        .collection::<Entry>("entries")
        .find(
            None,
            FindOptions::builder()
                .sort(doc! { "creationDate": -1 })
                .build(),
        )
        .await?;

    Ok(cursor
        .collect::<Vec<Result<Entry, _>>>()
        .await
        .into_iter()
        .flatten()
        .map(|entry| EntryDto::from_entry_with_authors(entry, &authors))
        .collect::<Vec<EntryDto>>())
}

#[server]
pub async fn get_entry(article: String) -> Result<Option<EntryDto>, ServerFnError> {
    use futures_util::StreamExt;
    use mongodb::bson::doc;

    let client =
        mongodb::Client::with_uri_str("mongodb://root:root@localhost/db?authSource=admin").await?;
    let database = client.database("blogDB");

    let cursor = database
        .collection::<AuthorDto>("users")
        .find(None, None)
        .await?;
    let authors = cursor
        .collect::<Vec<Result<AuthorDto, _>>>()
        .await
        .into_iter()
        .flatten()
        .collect::<Vec<AuthorDto>>();

    let filter = article.split('-').collect::<Vec<_>>().join(" ");

    let mut cursor = database
        .collection::<Entry>("entries")
        .find(doc! { "title": filter }, None)
        .await?;

    if let Some(entry) = cursor.next().await {
        Ok(Some(EntryDto::from_entry_with_authors(entry?, &authors)))
    } else {
        Ok(None)
    }
}
