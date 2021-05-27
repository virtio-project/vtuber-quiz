use actix_web::{Responder, HttpRequest, HttpResponse};
use chrono::{DateTime, Utc};
use chrono::serde::ts_milliseconds;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::Pool;


#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub password: String,
}

#[derive(sqlx::Type, Copy, Clone, Debug, Serialize, Eq, PartialEq)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
#[serde(rename_all = "snake_case")]
pub enum UserRole { Normal, Vtuber }

#[derive(sqlx::Type, Copy, Clone, Debug, Serialize, Eq, PartialEq)]
#[sqlx(type_name = "question_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum QuestionType { TrueFalse, MultiChoice, MultiAnswer }

#[derive(sqlx::Type, Copy, Clone, Debug, Serialize, Eq, PartialEq)]
#[sqlx(type_name = "vote_action", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum VoteAction { UpVote, DownVote, FlagOutdated, FlagIncorrect }

#[derive(Clone, Debug, Serialize, FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub role: UserRole,
    pub blocked: bool,
    pub reputation: i32,
    #[serde(with = "ts_milliseconds")]
    pub created: DateTime<Utc>,
    #[serde(with = "ts_milliseconds")]
    pub updated: DateTime<Utc>
}

#[derive(Clone, Debug, Serialize, FromRow)]
pub struct Question {
    pub id: i32,
    pub creator: i32,
    pub description: String,
    pub choices: Vec<String>,
    pub answer: Vec<i32>,
    #[sqlx(rename = "type")]
    pub question_type: QuestionType,
    pub draft: bool,
    #[serde(with = "ts_milliseconds")]
    pub created: DateTime<Utc>,
    #[serde(with = "ts_milliseconds")]
    pub updated: DateTime<Utc>
}

impl User {
    pub async fn create(db: &Pool, username: String, password: String) {
        let conn = db.acquire().await?;
        query!(r#"insert into "user""#)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::to_string;

    #[test]
    fn test_enum() {
        assert_eq!("\"normal\"", to_string(&UserRole::Normal).unwrap().as_str());
        assert_eq!("\"vtuber\"", to_string(&UserRole::Vtuber).unwrap().as_str());
        assert_eq!("\"true_false\"", to_string(&QuestionType::TrueFalse).unwrap().as_str());
        assert_eq!("\"multi_choice\"", to_string(&QuestionType::MultiChoice).unwrap().as_str());
        assert_eq!("\"multi_answer\"", to_string(&QuestionType::MultiAnswer).unwrap().as_str());
        assert_eq!("\"up_vote\"", to_string(&VoteAction::UpVote).unwrap().as_str());
        assert_eq!("\"down_vote\"", to_string(&VoteAction::DownVote).unwrap().as_str());
        assert_eq!("\"flag_outdated\"", to_string(&VoteAction::FlagOutdated).unwrap().as_str());
        assert_eq!("\"flag_incorrect\"", to_string(&VoteAction::FlagIncorrect).unwrap().as_str());
    }
}