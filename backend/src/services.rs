use actix_web::{post, web};
use actix_web::{Result, HttpResponse};

use crate::Pool;
use crate::hcaptcha::Hcaptcha;
use crate::models::{RegisterRequest, User};
use crate::error::Error;

#[post("/user")]
pub async fn register(req: web::Json<RegisterRequest>, pool: web::Data<Pool>, _hcaptcha: Hcaptcha) -> Result<HttpResponse> {
    let id = User::create(&pool, req.username.as_str(), req.password.as_str()).await?;
    Ok(HttpResponse::Ok().json(User::get_by_id(&pool, id).await?))
}

// #[post("/user/{username}/session")]
// async fn login() {
//
// }