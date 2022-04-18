#[macro_use]
extern crate log;
#[macro_use]
extern crate sqlx;

use std::borrow::Borrow;

use actix_files as fs;

use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;

use crate::config::Config;

mod bilibili;
mod config;
mod db;
mod error;
mod hcaptcha;
mod services;

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
            .wrap(config_cloned.host.cookie.session_middleware())
            .service(fs::Files::new("/swagger", "./public/swagger")
                .index_file("index.html")
            )
            .app_data(Data::new(config_cloned.hcaptcha.clone()))
            .app_data(Data::new(pool.clone()))
            .service(
                web::scope("/api")
                    .service(services::register)
                    .service(services::login)
                    .service(services::get_self)
                    .service(services::follow_user)
                    .service(services::unfollow_user)
                    .service(services::create_challenge_code)
                    .service(services::create_question)
                    .service(services::get_question)
                    .service(services::delete_question)
                    .service(services::update_question)
                    .service(services::apply_question_to_vtuber)
                    .service(services::remove_question_to_vtuber)
                    .service(services::get_question_applied)
                    .service(services::vote_to_question),
            )
    })
    .bind(&config.host.bind)?
    .run()
    .await?;

    Ok(())
}
