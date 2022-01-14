use crate::models::{CreatePasteReq, Paste};
use anyhow::Result;
use deadpool_postgres::Client;

pub async fn create_paste(client: &Client, cpr: CreatePasteReq) -> Result<String> {
    let paste = Paste::from_create_paste_req(cpr);
    let statement = client
        .prepare("INSERT INTO pastes VALUES ($1, $2, $3, $4)")
        .await?;

    client
        .execute(
            &statement,
            &[&paste.id, &paste.expired_at, &paste.language, &paste.data],
        )
        .await?;

    Ok(paste.id)
}

pub async fn read_paste(client: &Client, id: &str) -> Result<Paste> {
    let statement = client.prepare("SELECT * FROM pastes WHERE id=$1").await?;
    let row = client.query_one(&statement, &[&id]).await?;
    let res = Paste::from_row(&row);
    Ok(res)
}
