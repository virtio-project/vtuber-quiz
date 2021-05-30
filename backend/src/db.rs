use std::borrow::Cow;
use std::convert::TryFrom;

use rand::{thread_rng, RngCore, Rng};
use sqlx::PgPool;
use sqlx::postgres::PgDatabaseError;
use vtuber_quiz_commons::models::*;

use crate::error::Error;

#[derive(Debug, thiserror::Error)]
pub enum PgError {
    #[error("unique_violation")]
    UniqueViolation,
    #[error("check_violation")]
    CheckViolation,
    #[error("other error")]
    Others,
}

pub async fn create_user(pool: &PgPool, username: &str, password: &str) -> Result<i32, Error> {
    let hashed = hash_password(password.as_bytes());
    query!(r#"
insert into "user" (username, password)
values ( $1, $2 )
returning id"#,
            username,
            hashed
        )
        .fetch_one(pool)
        .await
        .map(|res| res.id)
        .map_err(|e: sqlx::Error| -> Error {
            match PgError::try_from(e) {
                Ok(pg) => match pg {
                    PgError::UniqueViolation => Error::ConflictUsername,
                    _ => pg.into(),
                },
                Err(e) => e
            }
        })
}

pub async fn get_user_by_username(pool: &PgPool, username: &str) -> Result<User, Error> {
    let user = query_as!(
            User,
            r#"
select id, username, password, challenge, blocked, role as "role: UserRole", reputation, created, updated
from "user"
where username = $1
limit 1
            "#,
            username
        )
        .fetch_one(pool)
        .await?;
    Ok(user)
}

pub async fn get_user_by_id(pool: &PgPool, id: i32) -> Result<User, Error> {
    let user = query_as!(
            User,
            r#"
select id, username, password, challenge, blocked, role as "role: UserRole", reputation, created, updated
from "user"
where id = $1
limit 1"#,
            id
        )
        .fetch_one(pool)
        .await?;
    Ok(user)
}

pub async fn create_or_replace_challenge(pool: &PgPool, id: i32) -> Result<String, Error> {
    loop {
        let challenge = generate_challenge_code();
        match query!(r#"update "user" set challenge = $1 where id = $2"#, &challenge, id).execute(pool).await {
            Ok(_) => return Ok(challenge),
            Err(e) => match PgError::try_from(e) {
                Ok(pg) => match pg {
                    PgError::UniqueViolation => continue,
                    _ => return Err(pg.into()),
                },
                Err(e) => return Err(e)
            },
        }
    }
}

pub async fn login(pool: &PgPool, username: &str, password: &str) -> Result<User, Error> {
    match get_user_by_username(pool, username).await {
        Ok(user) => {
            if verify_password(user.password.as_str(), password.as_bytes()) {
                Ok(user)
            } else {
                Err(Error::InvalidCredential)
            }
        }
        Err(Error::Sqlx(sqlx::Error::RowNotFound)) => Err(Error::InvalidCredential),
        Err(e) => Err(e),
    }
}

pub async fn follow(pool: &PgPool, from: i32, to: i32, private: bool) -> Result<(), Error> {
    match query!(r#"insert into following (follower, followee, private) values ($1, $2, $3)"#, from, to, private).execute(pool).await {
        Ok(_) => Ok(()),
        Err(e) => match PgError::try_from(e) {
            Ok(pg) => match pg {
                PgError::UniqueViolation => Ok(()),
                _ => Err(pg.into()),
            },
            Err(e) => Err(e)
        },
    }
}

pub async fn unfollow(pool: &PgPool, from: i32, to: i32) -> Result<(), Error> {
    query!(r#"delete from following where follower = $1 and followee = $2"#, from, to)
        .execute(pool).await
        .map(|_| ())?;
    Ok(())
}

pub async fn create_question(pool: &PgPool, creator: i32, question: QuestionCreationRequest) -> Result<i32, Error> {
    let content = &question.content;
    query!(r#"
insert into question (creator, description, choices, answer, "type", audiences, draft)
values ($1, $2, $3, $4, $5, $6, $7)
returning id"#,
            creator,
            content.description(),
            &content.choices(),
            &content.answer(),
            content.ty() as _,
            &question.audiences(),
            question.draft
        )
        .fetch_one(pool)
        .await
        .map(|res| res.id)
        .map_err(|e| e.into())
}

pub async fn get_question(pool: &PgPool, qid: i32) -> Result<Question, Error> {
    query_as!(
        Question,
        r#"
select id, creator, description, choices, answer,
       "type" as "question_type: QuestionType",
       audiences as "audiences: Vec<Audience>",
       draft, deleted, created, updated
from question
where id = $1"#,
        qid
    )
        .fetch_one(pool).await
        .map_err(|e| e.into())
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

fn verify_password(encoded: &str, password: &[u8]) -> bool {
    argon2::verify_encoded(encoded, password).unwrap()
}

fn generate_challenge_code() -> String {
    const CHALLENGE_LEN: usize = 7;
    const CHALLENGE_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = thread_rng();
    (0..CHALLENGE_LEN).map(|_| {
        let idx = rng.gen_range(0..CHALLENGE_CHARSET.len());
        CHALLENGE_CHARSET[idx] as char
    })
        .collect()
}

impl TryFrom<sqlx::Error> for PgError {
    type Error = Error;

    fn try_from(e: sqlx::Error) -> Result<Self, Self::Error> {
        use sqlx::Error::Database;
        match e {
            Database(e) => {
                // assert database is postgres
                Ok(e.downcast::<PgDatabaseError>().into())
            },
            _ => Err(e.into()),
        }
    }
}

impl From<Box<PgDatabaseError>> for PgError {
    fn from(e: Box<PgDatabaseError>) -> Self {
        use PgError::*;

        match e.code() {
            "23505" => UniqueViolation,
            "23514" => CheckViolation,
            _ => Others,
        }
    }
}