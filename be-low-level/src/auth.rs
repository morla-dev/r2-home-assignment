use actix_web::{web, HttpRequest, HttpResponse};
use regex::Regex;
use crate::error::AppError;
use crate::luck::LuckStore;
use crate::middleware::extract_token;
use crate::model::{LoginRequest, LoginResponse, LuckResponse};
use crate::token::TokenStore;

const REQUIRED_PASSWORD: &str = "r2isthebest";

fn is_valid_email(email: &str) -> bool {
    Regex::new(r"^[^@\s]+@[^@\s]+\.[^@\s]+$")
        .unwrap()
        .is_match(email)
}

pub async fn login(
    body: web::Json<LoginRequest>,
    token_store: web::Data<TokenStore>,
) -> Result<HttpResponse, AppError> {
    if !is_valid_email(&body.email) || body.password != REQUIRED_PASSWORD {
        return Err(AppError::Unauthorized("Invalid email or password".into()));
    }
    let token = token_store.generate();
    log::info!("User logged in: {}", body.email);
    Ok(HttpResponse::Ok().json(LoginResponse { token }))
}

pub async fn logout(
    req: HttpRequest,
    token_store: web::Data<TokenStore>,
) -> Result<HttpResponse, AppError> {
    let token = extract_token(&req)?;
    if !token_store.invalidate(&token) {
        return Err(AppError::Unauthorized("Invalid or expired token".into()));
    }
    log::info!("User logged out");
    Ok(HttpResponse::Ok().body("OK"))
}

pub async fn try_luck(
    req: HttpRequest,
    token_store: web::Data<TokenStore>,
    luck_store: web::Data<LuckStore>,
) -> Result<HttpResponse, AppError> {
    let token = extract_token(&req)?;
    if !token_store.is_valid(&token) {
        return Err(AppError::Unauthorized("Invalid or expired token".into()));
    }
    let won = luck_store.try_luck();
    log::info!("try_luck result: {}", won);
    Ok(HttpResponse::Ok().json(LuckResponse { win: won }))
}
