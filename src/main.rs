use std::env;
use std::str::FromStr;
use actix_files::Files;
use actix_session::{SessionMiddleware};
use actix_session::storage::CookieSessionStore;
use actix_web::cookie::Key;
use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode};
use tera::{Tera};
use dotenv::dotenv;

mod app;
use crate::app::*;




#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(move || {
        use actix_files::Files;
        let mut templates = Tera::new("templates/**/*").expect("errors in tera templates");
        templates.autoescape_on(vec!["tera"]);
        App::new()
            .wrap(Logger::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
                    .cookie_secure(false)
                    .build(),
            )
            .app_data(web::Data::new(templates))
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/Login").route(web::get().to(login)))
            // .service(Files::new("/static","static").show_files_listing())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}