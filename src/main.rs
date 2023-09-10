#![allow(unused_imports, unused_variables)]

#[macro_use]
extern crate rocket;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

mod cors;
mod handlers;
mod models;
mod persistance;

use anyhow::{anyhow, Result};
use cors::*;
use cors::*;
use dotenvy::dotenv;
use handlers::*;
use handlers::*;
use persistance::{questions_dao::{QuestionsDaoImpl, QuestionsDao}, answers_dao::{AnswersDaoImpl, AnswersDao}};
use rocket::{Build, Rocket};
use sqlx::postgres::PgPoolOptions;
use std::env;

use crate::models::Question;

#[launch]
async fn rocket() -> _ {
    pretty_env_logger::init();
    dotenv().ok();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set."))
        .await
        .expect("Failed to create Postgres connection pool!");

    let questions_dao =  QuestionsDaoImpl::new(pool.clone());
    let answers_dao = AnswersDaoImpl::new(pool.clone());

    rocket::build()
        .mount(
            "/",
            routes![
                create_question,
                read_questions,
                delete_question,
                create_answer,
                read_answers,
                delete_answer
            ],
        )
        .attach(CORS)
        .manage(Box::new(questions_dao) as Box::<dyn QuestionsDao + Send + Sync>) 
        .manage(Box::new(answers_dao) as Box::<dyn AnswersDao + Send + Sync>)

}
