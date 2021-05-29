use std::borrow::Cow;

use rand::{thread_rng, RngCore, Rng};
use sqlx::PgPool;
use vtuber_quiz_commons::models::*;

use crate::error::Error;

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
            match e {
                sqlx::Error::Database(e) => {
                    if e.code() == Some(Cow::Borrowed("23505")) {
                        Error::ConflictUsername
                    } else {
                        sqlx::Error::Database(e).into()
                    }
                }
                _ => e.into()
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
            // https://www.postgresql.org/docs/9.2/errcodes-appendix.html
            // 23505 unique_violation
            Err(sqlx::Error::Database(e)) => {
                if e.code() == Some(Cow::Borrowed("23505")) {
                    continue
                } else {
                    return Err(sqlx::Error::Database(e).into())
                }
            }
            Err(e) => return Err(e.into()),
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