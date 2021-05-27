use actix_web::{post, web};
use actix_web::{HttpResponse, Result};
use actix_session::Session;

use crate::error::Error;
use crate::hcaptcha::Hcaptcha;
use crate::models::{RegisterRequest, User};
use crate::Pool;


#[post("/user")]
pub async fn register(
    req: web::Json<RegisterRequest>,
    pool: web::Data<Pool>,
    session: Session,
    _hcaptcha: Hcaptcha,
) -> Result<HttpResponse> {
    let id = User::create(&pool, req.username.as_str(), req.password.as_str()).await?;
    session.insert("user", id)?;
    Ok(HttpResponse::Ok().json(User::get_by_id(&pool, id).await?))
}

// #[post("/user/{username}/session")]
// async fn login() {
//
// }
