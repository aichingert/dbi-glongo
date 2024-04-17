use leptos::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub use crate::database::{
    entities::Entry, 
    dtos::{EntryDto, AuthorDto, EntryApiDto}
};

#[derive(Error, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PostError {
    #[error("Invalid post ID.")]
    InvalidArticle,
    #[error("Post not found.")]
    PostNotFound,
    #[error("Server error.")]
    ServerError,
}

#[server(AddPost, "/api")]
pub async fn add_post(entry: EntryApiDto) -> Result<(), ServerFnError> {
    use base64::Engine;
    use base64::prelude::BASE64_STANDARD;

    let img: Vec<u8> = entry.image[1..entry.image.len() - 1].split(',').map(|n| n.parse::<u8>().unwrap()).collect();

    println!("{:?}", BASE64_STANDARD.encode(&img));

    Ok(())
}

/*
pub async fn get_cursor<T>(
    collection: &str, 
    doc: Option<Document>, 
    options: Option<FilterOptions>,
) -> Resutl<Cursor<T>, ServerFnError> {
    let client =
        mongodb::Client::with_uri_str("mongodb://root:root@localhost/db?authSource=admin").await?;
    let database = client.database("blogDB");

    database
        .collection::<T>(collection)
        .find(doc, options)
        .await?
}
*/

#[server]
pub async fn get_all_entries() -> Result<Vec<EntryDto>, ServerFnError> {
    use futures_util::StreamExt;
    use mongodb::{bson::doc, options::FindOptions};

    let client = mongodb::Client::with_uri_str("mongodb://root:root@localhost/db?authSource=admin").await?;
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

    println!("Hello");

    let cursor = database
        .collection::<Entry>("entries")
        .find(
            None,
            FindOptions::builder()
                .sort(doc! { "creationDate": -1 })
                .build(),
        )
        .await?;

    println!("Bello");

    let res = cursor
        .collect::<Vec<Result<Entry, _>>>()
        .await;

    let mut dtos = Vec::new();

    for ent in res.into_iter() {
        println!("{:?}", ent.is_ok());
        if let Ok(entity) = ent {
            let dto = entity._to_dto(&authors);
            dtos.push(dto);
        }
    }
    /*
        .into_iter()
        .flatten()
        .map(|entry| Entry::_to_dto(entry, &authors))
        .collect::<Vec<EntryDto>>();
    */

    println!("Cello");

    Ok(dtos)
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
        Ok(Some(Entry::_to_dto(entry?, &authors)))
    } else {
        Ok(None)
    }
}
