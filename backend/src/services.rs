use actix_session::Session;
use actix_web::{post, web};
use actix_web::{HttpResponse, Result};
use sqlx::PgPool;
use vtuber_quiz_commons::models::*;

use crate::db;
use crate::error::Error;
use crate::hcaptcha::Hcaptcha;


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

#[post("/user/self/bilbili/verify_code")]
pub async fn bind_bilbili(
    pool: web::Data<PgPool>,
    session: Session,
) -> Result<HttpResponse> {
    let id = session.get::<i32>("user").ok().flatten().ok_or(Error::InvalidCredential)?;
    let challenge = db::create_or_replace_challenge(&pool, id).await?;
    Ok(HttpResponse::Ok().json(ChallengeResponse::new(challenge.as_str())))
}