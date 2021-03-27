#[macro_use]
// extern crate log;
extern crate serde_json;

mod handlers;
mod routes;

use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use env_logger::{Builder, Target};
use std::time::Duration;

// use kv_log_macro::{debug, error, info, trace, warn};

use crate::handlers::setup;

use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    // let mut builder = Builder::from_default_env();
    // builder.target(Target::Stdout);
    // builder.init();

    let is_structured_json = std::env::var("RUST_STRUCTURED_LOG").unwrap_or(String::from("1"));

    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or(tracing_subscriber::EnvFilter::new("trace"));

    if is_structured_json == "1" {
        let subscriber = FmtSubscriber::builder()
            .json()
            .with_max_level(Level::TRACE)
            .with_current_span(false)
            .with_env_filter(env_filter)
            .finish();

        tracing::subscriber::set_global_default(subscriber)
            .expect("setting default subscriber failed");
    } else {
        let subscriber = FmtSubscriber::builder()
            .with_max_level(Level::TRACE)
            .with_env_filter(env_filter)
            .finish();

        tracing::subscriber::set_global_default(subscriber)
            .expect("setting default subscriber failed");
    }

    info!("starting up");

    let addr = std::env::var("ADDRESS").unwrap_or("0.0.0.0".to_owned());
    let port = std::env::var("PORT").unwrap_or("8080".to_owned());

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method();

        App::new()
            .wrap(cors)
            .wrap(tracing_actix_web::TracingLogger)
            // .wrap(middleware::Logger::default())
            .configure(setup)
            .default_service(web::to(|| async { "404" }))
    })
    .bind(format!("{}:{}", addr, port))?
    .run()
    .await
}
