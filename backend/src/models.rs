use chrono::{DateTime, Utc};
use chrono::serde::ts_milliseconds;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::Pool;
use crate::error::Error;
use rand::{thread_rng, RngCore};
use std::future::Future;
use std::borrow::Cow;

#[derive(Debug, Deserialize)]
pub struct RegRequest {
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
    #[serde(skip_serializing)]
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
    pub async fn create(pool: &Pool, username: &str, password: &str) -> Result<i32, Error> {
        let hashed = Self::hash_password(password.as_bytes());
        let id = query!(
            r#"
insert into "user" (username, password)
values ( $1, $2 )
returning id
            "#,
            username,
            hashed)
            .fetch_one(pool)
            .await;
        match id {
            Ok(id) => Ok(id.id),
            // https://www.postgresql.org/docs/9.2/errcodes-appendix.html
            // 23505 unique_violation
            Err(sqlx::Error::Database(e)) => if e.code() == Some(Cow::Borrowed("23505")) {
                Err(Error::ConflictUsername)
            } else {
                Err(sqlx::Error::Database(e).into())
            },
            Err(e) => Err(e.into())
        }

    }

    pub async fn get_by_username(pool: &Pool, username: &str) -> Result<Self, Error> {
        let user = query_as!(
            User,
            r#"
select id, username, password, blocked, role as "role: UserRole", reputation, created, updated
from "user"
where username = $1
limit 1
            "#,
            username)
            .fetch_one(pool)
            .await?;
        Ok(user)
    }

    pub async fn get_by_id(pool: &Pool, id: i32) -> Result<Self, Error> {
        let user = query_as!(
            User,
            r#"
select id, username, password, blocked, role as "role: UserRole", reputation, created, updated
from "user"
where id = $1
limit 1
            "#,
            id)
            .fetch_one(pool)
            .await?;
        Ok(user)
    }

    pub async fn login(pool: &Pool, username: &str, password: &str) -> Result<Self, Error> {
        match Self::get_by_username(pool, username).await {
            Ok(user) => if user.verify_password(password.as_bytes()) {
                Ok(user)
            } else {
                Err(Error::InvalidCredential)
            },
            Err(Error::Sqlx(sqlx::Error::RowNotFound)) => Err(Error::InvalidCredential),
            Err(e) => Err(e)
        }
    }

    fn hash_password(password: &[u8]) -> String {
        let mut rng = thread_rng();
        let mut salt = [0u8; 16];
        rng.fill_bytes(&mut salt);
        let config = argon2::Config {
            // ref: https://datatracker.ietf.org/doc/draft-irtf-cfrg-argon2/
            variant: argon2::Variant::Argon2id,
            ..Default::default()
        };
        argon2::hash_encoded(password, &salt, &config).unwrap()
    }

    fn verify_password(&self, password: &[u8]) -> bool {
        argon2::verify_encoded(self.password.as_str(), password).unwrap()
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