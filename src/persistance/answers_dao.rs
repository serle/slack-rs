#![allow(dead_code)]

use std::f32::consts::E;

use async_trait::async_trait;
use sqlx::{Error::Database as PgError, PgPool};

use crate::models::{postgres_error_codes, Answer, AnswerDetail, DBError};

#[async_trait]
pub trait AnswersDao {
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError>;
    async fn delete_answer(&self, answer_uuid: String) -> Result<(), DBError>;
    async fn get_answers(&self, question_uuid: String) -> Result<Vec<AnswerDetail>, DBError>;
}

pub struct AnswersDaoImpl {
    db: PgPool,
}

impl AnswersDaoImpl {
    pub fn new(db: PgPool) -> Self {
        AnswersDaoImpl { db }
    }
}

#[async_trait]
impl AnswersDao for AnswersDaoImpl {
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError> {
        let question_uuid = answer.question_uuid.as_str();
        let uuid = sqlx::types::Uuid::parse_str(&answer.question_uuid)
            .map_err(|e| DBError::InvalidUUID(format!(
                "Could not parse answer UUID: {}",
                answer.question_uuid
            )))?;
        let result = sqlx::query_as::<_, AnswerDetail>(
            r#"
                INSERT INTO answers ( question_uuid, content )
                VALUES ( $1, $2 )
                RETURNING *
            "#,
        )
        .bind(&uuid)
        .bind(&answer.content)
        .fetch_one(&self.db)
        .await
        .map_err(|e: sqlx::Error| match e {
            sqlx::Error::Database(e) => {
                if let Some(code) = e.code() {
                    if code.eq(postgres_error_codes::FOREIGN_KEY_VIOLATION) {
                        return DBError::InvalidUUID(format!(
                            "Invalid question uuid: {}",
                            uuid
                        ));
                    }
                }
                DBError::Other(Box::new(e))
            }
            _ => DBError::Other(Box::new(e))
        })?;
        Ok(result)
    }

    async fn delete_answer(&self, answer_uuid: String) -> Result<(), DBError> {
        let uuid = sqlx::types::Uuid::parse_str(&answer_uuid)
            .map_err(|e| DBError::InvalidUUID(answer_uuid.to_string()))?;

        sqlx::query(
            r#"
                DELETE FROM answers WHERE answer_uuid = $1
            "#
        )
        .bind(uuid)
        .execute(&self.db)
        .await
        .map_err(|e| DBError::Other(Box::new(e)))?;

        Ok(())
    }

    async fn get_answers(&self, question_uuid: String) -> Result<Vec<AnswerDetail>, DBError> {
        let uuid = sqlx::types::Uuid::parse_str(&question_uuid)
            .map_err(|e| DBError::InvalidUUID(question_uuid.to_string()))?;

        let result = sqlx::query_as::<_, AnswerDetail>(
                r#"
                    SELECT * FROM answers WHERE question_uuid = $1
                "#
            )
            .bind(uuid)
            .fetch_all(&self.db)
            .await
            .map_err(|e| DBError::Other(Box::new(e)))?;
    
        Ok(result)
    }
}
