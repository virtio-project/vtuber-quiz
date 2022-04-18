use actix_session::Session;
use actix_web::{delete, get, post, web};
use actix_web::{HttpResponse, Result};
use sqlx::PgPool;
use vtuber_quiz_commons::models::*;

use crate::db;
use crate::error::Error;
use crate::hcaptcha::Hcaptcha;
use std::str::FromStr;

#[post("/user")]
pub async fn register(
    req: web::Json<RegRequest>,
    pool: web::Data<PgPool>,
    session: Session,
    _hcaptcha: Hcaptcha,
) -> Result<HttpResponse> {
    let id = db::create_user(&pool, req.username.as_str(), req.password.as_str()).await?;
    session.insert("user", id)?;
    Ok(HttpResponse::Ok().json(db::get_user_by_id(&pool, id).await?))
}

#[post("/user/{username}/session")]
pub async fn login(
    username: web::Path<String>,
    req: web::Json<LoginRequest>,
    pool: web::Data<PgPool>,
    session: Session,
    _hcaptcha: Hcaptcha,
) -> Result<HttpResponse> {
    let user = db::login(&pool, username.as_str(), req.password.as_str()).await?;
    session.insert("user", user.id)?;
    Ok(HttpResponse::Ok().json(user))
}

#[post("/user/by-id/{id}/follow")]
pub async fn follow_user(
    id: web::Path<i32>,
    req: web::Json<FollowRequest>,
    pool: web::Data<PgPool>,
    session: Session,
) -> Result<HttpResponse> {
    let from = session
        .get::<i32>("user")
        .ok()
        .flatten()
        .ok_or(Error::InvalidCredential)?;
    db::follow(&pool, from, *id, req.private).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[delete("/user/by-id/{id}/follow")]
pub async fn unfollow_user(
    id: web::Path<i32>,
    pool: web::Data<PgPool>,
    session: Session,
) -> Result<HttpResponse> {
    let from = session
        .get::<i32>("user")
        .ok()
        .flatten()
        .ok_or(Error::InvalidCredential)?;
    db::unfollow(&pool, from, *id).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[get("/user/self")]
pub async fn get_self(pool: web::Data<PgPool>, session: Session) -> Result<HttpResponse> {
    let id = session
        .get::<i32>("user")
        .ok()
        .flatten()
        .ok_or(Error::InvalidCredential)?;
    let user = db::get_user_by_id(&pool, id).await?;
    Ok(HttpResponse::Ok().json(user))
}

#[post("/user/self/bilbili/verify_code")]
pub async fn create_challenge_code(
    pool: web::Data<PgPool>,
    session: Session,
) -> Result<HttpResponse> {
    let id = session
        .get::<i32>("user")
        .ok()
        .flatten()
        .ok_or(Error::InvalidCredential)?;
    let challenge = db::create_or_replace_challenge(&pool, id).await?;
    Ok(HttpResponse::Ok().json(ChallengeResponse::new(challenge.as_str())))
}

#[post("/user/vote/{qid}/{action}")]
pub async fn vote_to_question(
    path: web::Path<(i32, String)>,
    pool: web::Data<PgPool>,
    session: Session,
    _hcaptcha: Hcaptcha,
) -> Result<HttpResponse> {
    let user = session
        .get::<i32>("user")
        .ok()
        .flatten()
        .ok_or(Error::InvalidCredential)?;
    let (qid, action) = path.into_inner();
    let question = db::get_question(&pool, qid).await?;
    if question.creator == user || question.draft || question.deleted {
        return Ok(HttpResponse::BadRequest().finish());
    }
    if let Ok(action) = VoteAction::from_str(action.as_str()) {
        db::vote_to_question(&pool, user, qid, action).await?;
        Ok(HttpResponse::NoContent().finish())
    } else {
        Ok(HttpResponse::BadRequest().finish())
    }
}
