use actix_web::{http::StatusCode, post, web, HttpResponse};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::api_error::ApiError;

#[derive(Deserialize)]
pub struct PostKeyPressInfoInput {
    mac_addresses: Vec<Vec<i32>>,
    window_path: String,
    window_title: String,
    keyboard_layout: i64,
    key_pressed: String,
    created_at: DateTime<Utc>,
}

#[post("/key-presses")]
pub async fn post_key_press_info(
    pool: web::Data<Pool<Postgres>>,
    input: web::Json<PostKeyPressInfoInput>,
) -> Result<HttpResponse, ApiError> {
    println!("IN");
    sqlx::query::<sqlx::Postgres>("
        INSERT INTO key_press_info(id,mac_address,window_path,window_title,keyboard_layout,key_pressed,created_at)
        VALUES($1,$2,$3,$4,$5,$6,$7);
    ")
    .bind(Uuid::new_v4())
    .bind(input.mac_addresses[0].clone())
    .bind(input.window_path.clone())
    .bind(input.window_title.clone())
    .bind(input.keyboard_layout)
    .bind(input.key_pressed.clone())
    .bind(input.created_at)
    .execute(pool.as_ref())
    .await?;
    Ok(HttpResponse::build(StatusCode::OK).finish())
}
