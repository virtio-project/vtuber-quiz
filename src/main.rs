#[macro_use] extern crate log;

use std::borrow::Borrow;

use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, web};
use actix_session::CookieSession;
use sqlx::{Postgres, Pool};
use sqlx::postgres::PgPoolOptions;

mod config;
mod service;
mod models;
mod error;
mod hcaptcha;

use crate::config::Config;

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
            )
    })
    .bind(&config.host.bind)?
    .run()
    .await?;

    Ok(())
}
