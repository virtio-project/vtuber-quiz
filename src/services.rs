use actix_web::{post, web};

use crate::Pool;
use crate::hcaptcha::Hcaptcha;

#[post("/user")]
pub async fn register(pool: web::Data<Pool>, _hcaptcha: Hcaptcha) -> String {
    "hello".to_string()
}

// #[post("/user/{username}/session")]
// async fn login() {
//
// }