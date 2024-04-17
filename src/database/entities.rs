use serde::{Deserialize, Serialize};
use crate::database::dtos::{EntryDto, EntryApiDto, AuthorDto, CommentDto};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
    title: String,
    author: bson::oid::ObjectId,
    description: String,

    creation_date: bson::DateTime,
    edit_dates: Vec<bson::DateTime>,
    impression_count: u64,

    pub content: Content,

    comments_allowed: bool,
    categories: Vec<String>,
    comments: Vec<Comment>,
}

impl Entry {
    pub fn _new(entry: EntryApiDto) -> Self {
        Self {
            title: entry.title,
            author: bson::oid::ObjectId::parse_str(&entry.author).unwrap(),
            description: entry.description,
            creation_date: bson::DateTime::now(),
            edit_dates: vec![],
            impression_count: 0,
            content: Content {
                text: entry.text,
                links: vec![],
                images: vec![],
                coordinates: vec![],
            },
            comments_allowed: entry.comments_allowed.is_some(),
            categories: vec![],
            comments: vec![],
        }
    }
    pub fn _to_dto(self, authors: &Vec<AuthorDto>) -> EntryDto {
        let author = authors
            .iter()
            .find(|&author| author.id.unwrap() == self.author)
            .unwrap()
            .clone();

        let mut comments = Vec::new();

        for comment in self.comments.iter() {
            comments.push(CommentDto {
                text: comment.text.clone(),
                author: authors
                    .iter()
                    .find(|&author| author.id.unwrap() == comment.author)
                    .unwrap()
                    .clone(),
                creation_date: comment.creation_date,
            });
        }

        EntryDto {
            title: self.title,
            author,
            description: self.description,

            creation_date: self.creation_date,
            edit_dates: self.edit_dates,

            impression_count: self.impression_count,

            content: self.content,

            comments_allowed: self.comments_allowed,
            categories: self.categories,
            comments,
        }
    }
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
pub struct Comment {
    text: String,
    creation_date: bson::DateTime,
    author: bson::oid::ObjectId,
}
