use actix_web::{get, http::StatusCode, web, HttpResponse};
use serde::Serialize;
use sqlx::{postgres::PgRow, Pool, Postgres, Row};
use uuid::Uuid;

use crate::api_error::ApiError;

#[derive(Serialize)]
struct Window {
    id: Uuid,
    title: String,
}

#[get("/windows")]
pub async fn get_windows(pool: web::Data<Pool<Postgres>>) -> Result<HttpResponse, ApiError> {
    let windows = sqlx::query(r#"select * from "window""#)
        .map(|row: PgRow| Window {
            id: row.get_unchecked(0),
            title: row.get_unchecked(1),
        })
        .fetch_all(pool.as_ref())
        .await?;
    Ok(HttpResponse::build(StatusCode::OK).json(windows))
}
