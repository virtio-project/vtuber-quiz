use actix_session::Session;
// use actix_web::{delete, get, post, put};
use actix_web::{HttpResponse, Result};
use paperclip::actix::{delete, get, post, put, api_v2_operation, web};
use sqlx::PgPool;
use vtuber_quiz_commons::models::*;

use crate::db;
use crate::error::Error;
use crate::hcaptcha::Hcaptcha;

#[api_v2_operation(
    summary = "Create question",
    description = "It creates a question",
    operation_id = "create_question",
    consumes = "application/json",
    produces = "application/json",
    tags(Cats, Dogs, "Api reference"),
)]
#[post("/question")]
pub async fn create_question(
    req: web::Json<QuestionCreationRequest>,
    pool: web::Data<PgPool>,
    session: Session,
    _hcaptcha: Hcaptcha,
) -> Result<HttpResponse> {
    let creator = session
        .get::<i32>("user")
        .ok()
        .flatten()
        .ok_or(Error::InvalidCredential)?;
    let qid = db::create_question(&pool, creator, req.into_inner()).await?;
    Ok(HttpResponse::Ok().json(db::get_question(&pool, qid).await?))
}

#[api_v2_operation]
#[get("/question/{qid}")]
pub async fn get_question(qid: web::Path<i32>, pool: web::Data<PgPool>) -> Result<HttpResponse> {
    let question = db::get_question(&pool, *qid).await?;
    if question.deleted {
        Ok(HttpResponse::NotFound().finish())
    } else {
        Ok(HttpResponse::Ok().json(question))
    }
}

#[api_v2_operation]
#[delete("/question/{qid}")]
pub async fn delete_question(
    qid: web::Path<i32>,
    pool: web::Data<PgPool>,
    session: Session,
) -> Result<HttpResponse> {
    let user = session
        .get::<i32>("user")
        .ok()
        .flatten()
        .ok_or(Error::InvalidCredential)?;
    let question = db::get_question(&pool, *qid).await?;
    if question.creator == user {
        db::delete_question(&pool, *qid).await?;
        Ok(HttpResponse::NoContent().finish())
    } else {
        Ok(HttpResponse::Unauthorized().finish())
    }
}

/// Fields that can be updated:
/// - description
/// - choices
/// - answer
/// - audiences
/// - draft
/// Fields that will be ignored in update:
/// - id
/// - creator
/// - question_type
/// - deleted
/// - created
/// Fields that will be overwritten by update:
/// - updated
#[api_v2_operation]
#[put("/question/{qid}")]
pub async fn update_question(
    req: web::Json<Question>,
    qid: web::Path<i32>,
    pool: web::Data<PgPool>,
    session: Session,
) -> Result<HttpResponse> {
    let user = session
        .get::<i32>("user")
        .ok()
        .flatten()
        .ok_or(Error::InvalidCredential)?;
    let question = req.into_inner();
    if *qid != question.id || !question.is_valid() {
        return Ok(HttpResponse::BadRequest().finish());
    }
    let origin_question = db::get_question(&pool, *qid).await?;
    if origin_question.creator != user {
        return Ok(HttpResponse::Unauthorized().finish());
    }
    db::update_question(&pool, question).await?;
    Ok(HttpResponse::Ok().json(db::get_question(&pool, *qid).await?))
}

#[api_v2_operation]
#[post("/question/{qid}/apply/{uid}")]
pub async fn apply_question_to_vtuber(
    path: web::Path<(i32, i32)>,
    pool: web::Data<PgPool>,
    session: Session,
) -> Result<HttpResponse> {
    let user = session
        .get::<i32>("user")
        .ok()
        .flatten()
        .ok_or(Error::InvalidCredential)?;
    let (qid, uid) = path.into_inner();
    let question = db::get_question(&pool, qid).await?;
    if question.creator != user {
        return Ok(HttpResponse::Unauthorized().finish());
    }
    let vtuber = db::get_user_by_id(&pool, uid).await?;
    if vtuber.role != UserRole::Vtuber {
        return Ok(HttpResponse::BadRequest().finish());
    }
    db::apply_question_to_vtuber(&pool, qid, uid).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[api_v2_operation]
#[delete("/question/{qid}/apply/{uid}")]
pub async fn remove_question_to_vtuber(
    path: web::Path<(i32, i32)>,
    pool: web::Data<PgPool>,
    session: Session,
) -> Result<HttpResponse> {
    let user = session
        .get::<i32>("user")
        .ok()
        .flatten()
        .ok_or(Error::InvalidCredential)?;
    let (qid, uid) = path.into_inner();
    let question = db::get_question(&pool, qid).await?;
    if question.creator != user {
        return Ok(HttpResponse::Unauthorized().finish());
    }
    let vtuber = db::get_user_by_id(&pool, uid).await?;
    if vtuber.role != UserRole::Vtuber {
        return Ok(HttpResponse::BadRequest().finish());
    }
    db::remove_question_to_vtuber(&pool, qid, uid).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[api_v2_operation]
#[get("/question/{qid}/apply")]
pub async fn get_question_applied(
    qid: web::Path<i32>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse> {
    let question = db::get_question(&pool, *qid).await?;
    if question.draft || question.deleted {
        return Ok(HttpResponse::NotFound().finish());
    }
    Ok(HttpResponse::Ok().json(db::get_question_applied(&pool, *qid).await))
}
