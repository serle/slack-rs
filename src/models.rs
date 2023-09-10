#![allow(dead_code)]

use std::ops::Deref;
use serde::{Deserialize, Deserializer, Serialize};
use sqlx::{postgres::PgRow, types::Uuid, FromRow, Row};
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct Question {
    pub question: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct QuestionDetail {
    pub question_uuid: String,
    pub title: String,
    pub description: String,
    pub created_at: String,
}

impl<'r> FromRow<'r, PgRow> for QuestionDetail {
    fn from_row(row: &'r PgRow) -> sqlx::Result<Self, sqlx::Error> {
        let question_uuid = row.try_get::<sqlx::types::Uuid, _>("question_uuid")?.to_string();
        let title = row.try_get::<String, _>("title")?;
        let description =  row.try_get::<String, _>("description")?;
        let created_at = row.try_get::<sqlx::types::time::PrimitiveDateTime, _>("created_at")?.to_string();

        Ok(Self {
            question_uuid,
            title,
            description,
            created_at,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuestionId {
    pub question_uuid: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Answer {
    pub question_uuid: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AnswerDetail {
    pub answer_uuid: String,
    pub question_uuid: String,
    pub content: String,
    pub created_at: String,
}

impl<'r> FromRow<'r, PgRow> for AnswerDetail {
    fn from_row(row: &'r PgRow) -> sqlx::Result<Self, sqlx::Error> {
        let answer_uuid = row
            .try_get::<sqlx::types::Uuid, _>("answer_uuid")?
            .to_string();
        let question_uuid = row
            .try_get::<sqlx::types::Uuid, _>("question_uuid")?
            .to_string();
        let content = row.try_get::<String, _>("content")?;
        let created_at = row
            .try_get::<sqlx::types::time::PrimitiveDateTime, _>("created_at")?
            .to_string();

        Ok(Self {
            answer_uuid,
            question_uuid,
            content,
            created_at,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AnswerId {
    pub answer_uuid: String,
}

#[derive(Error, Debug)]
pub enum DBError {
    #[error("Invalid UUID provided: {0}")]
    InvalidUUID(String),
    #[error("Database error occurred")]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}

// source: https://www.postgresql.org/docs/current/errcodes-appendix.html
pub mod postgres_error_codes {
    pub const FOREIGN_KEY_VIOLATION: &str = "23503";
}
