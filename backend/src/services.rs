use actix_web::{post, web};

use crate::Pool;
use crate::hcaptcha::Hcaptcha;
use crate::models::RegisterRequest;

#[post("/user")]
pub async fn register(req: web::Json<RegisterRequest>, pool: web::Data<Pool>, _hcaptcha: Hcaptcha) -> String {
    format!("hello {:?}", req)
}

// #[post("/user/{username}/session")]
// async fn login() {
//
// }