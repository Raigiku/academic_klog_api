use actix_web::{get, http::StatusCode, web, HttpResponse};
use serde::Serialize;
use sqlx::{postgres::PgRow, Pool, Postgres, Row};
use uuid::Uuid;

use crate::api_error::ApiError;

#[derive(Serialize)]
struct Program {
    id: Uuid,
    name: String,
}

#[get("/programs")]
pub async fn get_programs(pool: web::Data<Pool<Postgres>>) -> Result<HttpResponse, ApiError> {
    let programs = sqlx::query(r#"select * from program"#)
        .map(|row: PgRow| Program {
            id: row.get_unchecked(0),
            name: row.get_unchecked(1),
        })
        .fetch_all(pool.as_ref())
        .await?;
    Ok(HttpResponse::build(StatusCode::OK).json(programs))
}
