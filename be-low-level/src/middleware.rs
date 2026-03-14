use actix_web::HttpRequest;
use crate::error::AppError;

pub fn extract_token(req: &HttpRequest) -> Result<String, AppError> {
    let header = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| AppError::Unauthorized("Missing Authorization header".into()))?;

    if !header.starts_with("Bearer ") {
        return Err(AppError::Unauthorized("Invalid Authorization format".into()));
    }

    Ok(header[7..].to_string())
}
