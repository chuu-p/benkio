use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{flashcards, reviews};

#[derive(Queryable, Selectable, Insertable, Debug, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::flashcards)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Flashcard {
    pub id: i32,
    pub content_front: String,
    pub content_back: String,
    pub interval: i32,
    pub next_review: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = flashcards)]
pub struct NewCard<'a> {
    pub content_front: &'a str,
    pub content_back: &'a str,
    pub interval: i32,
    pub next_review: chrono::NaiveDateTime,
}

#[derive(Queryable, Selectable, Insertable, Debug, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::reviews)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Review {
    pub id: i32,
    pub card_id: i32,
    pub review_date: NaiveDateTime,
    pub duration_front: f32,
    pub duration_back: f32,
    pub correct: bool,
    pub interval: i32,
}

#[derive(Insertable)]
#[diesel(table_name = reviews)]
pub struct NewReview {
    pub card_id: i32,
    pub review_date: NaiveDateTime,
    pub duration_front: f32,
    pub duration_back: f32,
    pub correct: bool,
    pub interval: i32,
}
