use std::future::Future;
use std::net::IpAddr;
use std::pin::Pin;
use std::str::FromStr;

use actix_web::dev::Payload;
use actix_web::web::Data;
use actix_web::{FromRequest, HttpRequest};

use crate::error::Error;

pub struct Hcaptcha {
    _private: (),
}

#[derive(thiserror::Error, Copy, Clone, Debug)]
pub enum HcaptchaError {
    #[error("missing hCaptcha challenge response header")]
    Missing,
    #[error("insufficient information to verify hCaptcha challenge")]
    InsufficientInformation,
    #[error("invalid hCaptcha challenge response header")]
    Invalid,
}

impl FromRequest for Hcaptcha {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        if cfg!(feature = "bypass-hcaptcha") {
            return Box::pin(async { Ok(Hcaptcha { _private: () }) });
        }
        let response: Option<&str> = req
            .headers()
            .get("X-HCAPTCHA-KEY")
            .and_then(|key| key.to_str().ok());
        if response.is_none() {
            return Box::pin(async { Err(HcaptchaError::Missing.into()) });
        }
        let user_ip = IpAddr::from_str(req.connection_info().realip_remote_addr().unwrap());
        if user_ip.is_err() {
            return Box::pin(async { Err(HcaptchaError::InsufficientInformation.into()) });
        }
        let config = req.app_data::<Data<crate::config::HCaptcha>>().unwrap();
        let mut hc = hcaptcha::Hcaptcha::new(config.secret.as_str(), response.unwrap())
            .set_site_key(config.site_key.as_str())
            .set_user_ip(&user_ip.unwrap());
        Box::pin(async move {
            hc.verify()
                .await
                .map(|_| Hcaptcha { _private: () })
                .map_err(|e| {
                    error!("Hcaptcha failed: {:?}", e);
                    HcaptchaError::Invalid.into()
                })
        })
    }
}
