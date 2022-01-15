mod config;
mod db;
mod handlers;
mod models;

use crate::handlers::{create_paste_handler, read_paste_handler};
use actix_cors::Cors;
use actix_web::{web::Data, App, HttpServer};
use actix_web::middleware::Logger;
use dotenv::dotenv;
use tokio_postgres::NoTls;
use env_logger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let cfg = crate::config::Config::from_env().expect("could not load config");
    let pool = cfg.pg.create_pool(None, NoTls).unwrap();
    let data = Data::new(pool);
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let server = HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(Data::clone(&data))
            .service(create_paste_handler)
            .service(read_paste_handler)
    })
    .bind(cfg.server_addr.to_string())?
    .run();

    server.await
}
