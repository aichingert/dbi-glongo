use leptos::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub use crate::database::{
    entities::{Entry, Content},
    dtos::{EntryDto, AuthorDto, EntryApiDto}
};

use cfg_if::cfg_if;
cfg_if! {
if #[cfg(feature = "ssr")] {
    use mongodb::{Cursor, bson::{doc, Document}, options::FindOptions};
    use futures_util::StreamExt;

    use base64::Engine;
    use base64::prelude::BASE64_STANDARD;

    pub async fn get_cursor<T>(
        collection: &str, 
        doc: Option<Document>, 
        options: Option<FindOptions>,
    ) -> Result<Cursor<T>, ServerFnError> {
        Ok(mongodb::Client::with_uri_str("mongodb://root:root@localhost/db?authSource=admin")
            .await?
            .database("blogDB")
            .collection::<T>(collection)
            .find(doc, options)
            .await?
        )
    }
}
}
#[derive(Error, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)] pub enum PostError {
    #[error("Invalid post ID.")]
    InvalidArticle,
    #[error("Post not found.")]
    PostNotFound,
    #[error("Server error.")]
    ServerError,
}

#[server(AddPost, "/api")]
pub async fn add_post(entry: EntryApiDto) -> Result<(), ServerFnError> { 
    let image = entry.image.clone();

    // doc!{ "$inc": { "impressionCount": 1 } }
    let author = get_cursor::<AuthorDto>("users", Some(doc! { "firstname": "Guest" }), None)
        .await?
        .next()
        .await;

    //println!("{:?}", entry.author);
    let mut entry = Entry::_new(entry);
    entry.author = author.unwrap().unwrap().id.unwrap();

if let Some(raw) = image {
        let img: Vec<u8> = raw[1..raw.len() - 1].split(',').map(|n| n.parse::<u8>().unwrap()).collect();
        entry.content.images.push(BASE64_STANDARD.encode(&img));
    }

    mongodb::Client::with_uri_str("mongodb://root:root@localhost/db?authSource=admin")
        .await?
        .database("blogDB")
        .collection("entries")
        .insert_one(entry, None)
        .await?;

    Ok(())
}

#[server]
pub async fn get_all_entries() -> Result<Vec<EntryDto>, ServerFnError> {
    let authors = get_cursor::<AuthorDto>("users", None, None)
        .await?
        .collect::<Vec<Result<AuthorDto, _>>>()
        .await
        .into_iter()
        .flatten()
        .collect::<Vec<AuthorDto>>();

    Ok(get_cursor::<Entry>("entries", None, 
        Some(FindOptions::builder().sort(doc! { "creationDate": -1 }).build()))
        .await?
        .collect::<Vec<Result<Entry, _>>>()
        .await
        .into_iter()
        .flatten()
        .filter_map(|entry| Entry::_to_dto(entry, &authors))
        .collect::<Vec<EntryDto>>())
}

#[server]
pub async fn get_entry(article: String) -> Result<Option<EntryDto>, ServerFnError> {
    let  authors= get_cursor::<AuthorDto>("users", None, None)
        .await?
        .collect::<Vec<Result<AuthorDto, _>>>()
        .await
        .into_iter()
        .flatten()
        .collect::<Vec<AuthorDto>>();

    let filter = article.split('-').collect::<Vec<_>>().join(" ");

    if let Some(res) = mongodb::Client::with_uri_str("mongodb://root:root@localhost/db?authSource=admin")
        .await?
        .database("blogDB")
        .collection::<Entry>("entries")
        .find_one_and_update(doc! { "title": &filter }, doc!{ "$inc": { "impressionCount": 1 } }, None)
        .await?
    {
        Ok(res._to_dto(&authors))
    } else {
        Ok(None)
    }
}
