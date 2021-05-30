use actix_session::Session;
use actix_web::{get, post, delete, web};
use actix_web::{HttpResponse, Result};
use sqlx::PgPool;
use vtuber_quiz_commons::models::*;

use crate::db;
use crate::error::Error;
use crate::hcaptcha::Hcaptcha;

#[post("/question")]
pub async fn create_question(
    req: web::Json<QuestionCreationRequest>,
    pool: web::Data<PgPool>,
    session: Session,
    _hcaptcha: Hcaptcha,
) -> Result<HttpResponse> {
    let creator = session.get::<i32>("user").ok().flatten().ok_or(Error::InvalidCredential)?;
    let qid = db::create_question(&pool, creator, req.into_inner()).await?;
    Ok(HttpResponse::Ok().json(db::get_question(&pool, qid).await?))
}

#[get("/question/{qid}")]
pub async fn get_question(
    qid: web::Path<i32>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(db::get_question(&pool, *qid).await?))
}