mod api_error;
mod key_press_info;
mod program;
mod window;

use actix_web::{App, HttpServer};
use api_error::ApiError;
use key_press_info::post_key_press_info;
use sqlx::postgres::PgPoolOptions;

use program::get_programs;
use window::get_windows;

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
            .service(post_key_press_info)
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await?;

    Ok(())
}
