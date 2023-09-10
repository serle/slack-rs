#![allow(dead_code)]

use async_trait::async_trait;
use sqlx::{PgPool, FromRow, postgres::PgRow};

use crate::models::{DBError, Question, QuestionDetail};

#[async_trait]
pub trait QuestionsDao {
    async fn create_question(&self, question: Question) -> Result<QuestionDetail, DBError>;
    async fn delete_question(&self, question_uuid: String) -> Result<(), DBError>;
    async fn get_questions(&self) -> Result<Vec<QuestionDetail>, DBError>;
}

pub struct QuestionsDaoImpl {
    db: PgPool,
}

impl QuestionsDaoImpl {
    pub fn new(db: PgPool) -> Self {
        QuestionsDaoImpl { db }
    }
}

#[async_trait]
impl QuestionsDao for QuestionsDaoImpl {
    async fn create_question(&self, question: Question) -> Result<QuestionDetail, DBError> {
        let result = sqlx::query_as::<_, QuestionDetail>(
            r#"
                INSERT INTO questions ( title, description ) 
                VALUES ( $1, $2 ) 
                RETURNING *
            "#,
        )
        .bind(question.question)
        .bind(question.description)
        .fetch_one(&self.db)
        .await
        .map_err(|e| DBError::Other(Box::new(e)))?;
        Ok(result)
    }

    async fn delete_question(&self, question_uuid: String) -> Result<(), DBError> {
        let uuid = sqlx::types::Uuid::parse_str(&question_uuid)
            .map_err(|e| DBError::InvalidUUID(question_uuid))?;

        sqlx::query(
            r#"
                DELETE FROM questions WHERE question_uuid = $1
            "#,
        )
        .bind(&uuid)
        .execute(&self.db)
        .await
        .map_err(|e| DBError::Other(Box::new(e)))?;

        Ok(())
    }

    async fn get_questions(&self) -> Result<Vec<QuestionDetail>, DBError> {
        let questions = sqlx::query_as::<_, QuestionDetail>(
            r#"
                SELECT * FROM questions
            "#,
        )
        .fetch_all(&self.db)
        .await
        .map_err(|e: sqlx::Error| DBError::Other(Box::new(e)))?;

        Ok(questions)
    }
}
