use crate::db::{create_paste, read_paste};
use crate::models::CreatePasteReq;
use actix_web::web::{Data, Json, Path};
use actix_web::{get, post, HttpResponse};
use deadpool_postgres::Pool;

#[post("/api/create")]
pub async fn create_paste_handler(cpr: Json<CreatePasteReq>, db_pool: Data<Pool>) -> HttpResponse {
    let client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");
    let res = create_paste(&client, cpr.into_inner()).await;
    match res {
        Ok(id) => HttpResponse::Created().json(id),
        Err(e) => HttpResponse::NotAcceptable().body(format!("{}", e)),
    }
}

#[get("/api/read/{id}")]
pub async fn read_paste_handler(id: Path<(String,)>, db_pool: Data<Pool>) -> HttpResponse {
    let client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");

    let res = read_paste(&client, &id.into_inner().0).await;
    match res {
        Ok(paste) => HttpResponse::Ok().json(paste),
        Err(e) => HttpResponse::NotFound().body(format!("{}", e)),
    }
}
