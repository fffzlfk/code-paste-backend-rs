mod config;
mod db;
mod models;
mod handlers;

use crate::handlers::{create_paste_handler, read_paste_handler};
use actix_web::{web::Data, App, HttpServer};
use dotenv::dotenv;
use tokio_postgres::NoTls;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let cfg = crate::config::Config::from_env().expect("could not load config");

    let pool = cfg.pg.create_pool(None, NoTls).unwrap();

    let data = Data::new(pool);

    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::clone(&data))
            .service(create_paste_handler)
            .service(read_paste_handler)
    })
    .bind(cfg.server_addr.to_string())?
    .run();

    server.await
}
