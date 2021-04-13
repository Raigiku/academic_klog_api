use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum ApiError {
    InternalError { error_type: String, message: String },
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::InternalError {
                error_type,
                message,
            } => write!(f, "{} {}", error_type, message),
        }
    }
}

impl std::error::Error for ApiError {}

impl actix_web::error::ResponseError for ApiError {
    fn error_response(&self) -> actix_web::HttpResponse {
        actix_web::dev::HttpResponseBuilder::new(self.status_code())
            .set_header(actix_web::http::header::CONTENT_TYPE, "application/json")
            .json(self)
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }
}

impl From<std::env::VarError> for ApiError {
    fn from(err: std::env::VarError) -> Self {
        ApiError::InternalError {
            error_type: "std::env::VarError".to_string(),
            message: err.to_string(),
        }
    }
}

impl From<std::io::Error> for ApiError {
    fn from(err: std::io::Error) -> Self {
        ApiError::InternalError {
            error_type: "std::io::Error".to_string(),
            message: err.to_string(),
        }
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(err: sqlx::Error) -> Self {
        ApiError::InternalError {
            error_type: "sqlx::Error".to_string(),
            message: err.to_string(),
        }
    }
}
