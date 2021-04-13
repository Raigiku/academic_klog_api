mod api_error;

use actix_web::{get, http::StatusCode, web, App, HttpResponse, HttpServer};
use api_error::ApiError;
use serde::Serialize;
use sqlx::{
    postgres::{PgPoolOptions, PgRow},
    Pool, Postgres, Row,
};
use uuid::Uuid;

#[derive(Serialize)]
struct Program {
    id: Uuid,
    name: String,
}

#[get("/programs")]
async fn get_programs(pool: web::Data<Pool<Postgres>>) -> Result<HttpResponse, ApiError> {
    let programs = sqlx::query(r#"select * from program"#)
        .map(|row: PgRow| Program {
            id: row.get_unchecked(0),
            name: row.get_unchecked(1),
        })
        .fetch_all(pool.as_ref())
        .await?;
    Ok(HttpResponse::build(StatusCode::OK).json(programs))
}

#[derive(Serialize)]
struct Window {
    id: Uuid,
    title: String,
}

#[get("/windows")]
async fn get_windows(pool: web::Data<Pool<Postgres>>) -> Result<HttpResponse, ApiError> {
    let windows = sqlx::query(r#"select * from "window""#)
        .map(|row: PgRow| Window {
            id: row.get_unchecked(0),
            title: row.get_unchecked(1),
        })
        .fetch_all(pool.as_ref())
        .await?;
    Ok(HttpResponse::build(StatusCode::OK).json(windows))
}

#[actix_web::main]
async fn main() -> Result<(), ApiError> {
    let pg_uri = std::env::var("RKR_PG_URI")?;
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(&pg_uri)
        .await?;

    let port = if let Ok(port) = std::env::var("PORT") {
        port
    } else {
        "8080".to_string()
    };

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(get_programs)
            .service(get_windows)
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await?;

    Ok(())
}
