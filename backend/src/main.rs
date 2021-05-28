#[macro_use]
extern crate log;
#[macro_use]
extern crate sqlx;

use std::borrow::Borrow;

use actix_session::CookieSession;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;
use sqlx::Postgres;

mod config;
mod error;
mod hcaptcha;
mod models;
mod services;
mod bilibili;

use crate::config::Config;
type Pool = sqlx::Pool<Postgres>;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();
    let config = Config::default();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_with(config.database.borrow().into())
        .await?;

    let config_cloned = config.clone();
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(CookieSession::from(&config_cloned.host.cookie))
            .data(config_cloned.hcaptcha.clone())
            .data(pool.clone())
            .service(
                web::scope("/api")
                    .service(services::register)
                    .service(services::login),
            )
    })
    .bind(&config.host.bind)?
    .run()
    .await?;

    Ok(())
}
