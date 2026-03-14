use actix_web::{test, web, App};
use serde_json::json;

use crate::auth;
use crate::luck::LuckStore;
use crate::token::TokenStore;

macro_rules! build_app {
    () => {{
        let token_store = web::Data::new(TokenStore::new());
        let luck_store = web::Data::new(LuckStore::new());

        App::new()
            .app_data(token_store)
            .app_data(luck_store)
            .route("/api/login", web::post().to(auth::login))
            .route("/api/logout", web::post().to(auth::logout))
            .route("/api/try_luck", web::post().to(auth::try_luck))
    }};
}

// ──────────────────────────────────────────────
// LOGIN
// ──────────────────────────────────────────────

#[actix_web::test]
async fn test_login_success() {
    let app = test::init_service(build_app!()).await;

    let req = test::TestRequest::post()
        .uri("/api/login")
        .set_json(json!({ "email": "mor@test.com", "password": "r2isthebest" }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body["token"].is_string());
    assert!(!body["token"].as_str().unwrap().is_empty());
}

#[actix_web::test]
async fn test_login_wrong_password() {
    let app = test::init_service(build_app!()).await;

    let req = test::TestRequest::post()
        .uri("/api/login")
        .set_json(json!({ "email": "mor@test.com", "password": "wrongpass" }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 401);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body["error"].is_string());
}

#[actix_web::test]
async fn test_login_invalid_email() {
    let app = test::init_service(build_app!()).await;

    let req = test::TestRequest::post()
        .uri("/api/login")
        .set_json(json!({ "email": "not-an-email", "password": "r2isthebest" }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 401);
}

#[actix_web::test]
async fn test_login_missing_fields() {
    let app = test::init_service(build_app!()).await;

    let req = test::TestRequest::post()
        .uri("/api/login")
        .set_json(json!({ "email": "mor@test.com" }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
}

// ──────────────────────────────────────────────
// LOGOUT
// ──────────────────────────────────────────────

#[actix_web::test]
async fn test_logout_success() {
    let app = test::init_service(build_app!()).await;

    let login_req = test::TestRequest::post()
        .uri("/api/login")
        .set_json(json!({ "email": "mor@test.com", "password": "r2isthebest" }))
        .to_request();
    let login_resp: serde_json::Value = test::call_and_read_body_json(&app, login_req).await;
    let token = login_resp["token"].as_str().unwrap().to_string();

    let logout_req = test::TestRequest::post()
        .uri("/api/logout")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let resp = test::call_service(&app, logout_req).await;
    assert_eq!(resp.status(), 200);
}

#[actix_web::test]
async fn test_logout_invalid_token() {
    let app = test::init_service(build_app!()).await;

    let req = test::TestRequest::post()
        .uri("/api/logout")
        .insert_header(("Authorization", "Bearer fake-token-123"))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 401);
}

#[actix_web::test]
async fn test_logout_missing_header() {
    let app = test::init_service(build_app!()).await;

    let req = test::TestRequest::post()
        .uri("/api/logout")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 401);
}

#[actix_web::test]
async fn test_logout_double_logout() {
    let app = test::init_service(build_app!()).await;

    let login_req = test::TestRequest::post()
        .uri("/api/login")
        .set_json(json!({ "email": "mor@test.com", "password": "r2isthebest" }))
        .to_request();
    let login_resp: serde_json::Value = test::call_and_read_body_json(&app, login_req).await;
    let token = login_resp["token"].as_str().unwrap().to_string();

    let logout_req = test::TestRequest::post()
        .uri("/api/logout")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, logout_req).await;
    assert_eq!(resp.status(), 200);

    let logout_req2 = test::TestRequest::post()
        .uri("/api/logout")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp2 = test::call_service(&app, logout_req2).await;
    assert_eq!(resp2.status(), 401);
}

// ──────────────────────────────────────────────
// TRY LUCK
// ──────────────────────────────────────────────

#[actix_web::test]
async fn test_try_luck_success() {
    let app = test::init_service(build_app!()).await;

    let login_req = test::TestRequest::post()
        .uri("/api/login")
        .set_json(json!({ "email": "mor@test.com", "password": "r2isthebest" }))
        .to_request();
    let login_resp: serde_json::Value = test::call_and_read_body_json(&app, login_req).await;
    let token = login_resp["token"].as_str().unwrap().to_string();

    let req = test::TestRequest::post()
        .uri("/api/try_luck")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body["win"].is_boolean());
}

#[actix_web::test]
async fn test_try_luck_without_token() {
    let app = test::init_service(build_app!()).await;

    let req = test::TestRequest::post()
        .uri("/api/try_luck")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 401);
}

#[actix_web::test]
async fn test_try_luck_invalid_token() {
    let app = test::init_service(build_app!()).await;

    let req = test::TestRequest::post()
        .uri("/api/try_luck")
        .insert_header(("Authorization", "Bearer not-a-real-token"))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 401);
}

#[actix_web::test]
async fn test_try_luck_token_invalidated_after_logout() {
    let app = test::init_service(build_app!()).await;

    let login_req = test::TestRequest::post()
        .uri("/api/login")
        .set_json(json!({ "email": "mor@test.com", "password": "r2isthebest" }))
        .to_request();
    let login_resp: serde_json::Value = test::call_and_read_body_json(&app, login_req).await;
    let token = login_resp["token"].as_str().unwrap().to_string();

    let logout_req = test::TestRequest::post()
        .uri("/api/logout")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    test::call_service(&app, logout_req).await;

    let req = test::TestRequest::post()
        .uri("/api/try_luck")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 401);
}

// ──────────────────────────────────────────────
// LUCK STORE UNIT TESTS
// ──────────────────────────────────────────────

#[cfg(test)]
mod luck_tests {
    use crate::luck::{LuckStore, DAILY_WIN_THRESHOLD};

    #[test]
    fn test_try_luck_returns_bool() {
        let store = LuckStore::new();
        let result = store.try_luck();
        assert!(result == true || result == false);
    }

    #[test]
    fn test_win_rate_drops_after_threshold() {
        let store = LuckStore::new();

        let mut wins_before = 0u32;
        while wins_before < DAILY_WIN_THRESHOLD {
            if store.try_luck() { wins_before += 1; }
        }

        let mut wins_after = 0u32;
        let attempts = 300u32;
        for _ in 0..attempts {
            if store.try_luck() { wins_after += 1; }
        }

        let win_rate = wins_after as f64 / attempts as f64;
        assert!(
            win_rate < 0.65,
            "Win rate after threshold ({:.2}) should be reduced, expected ~0.4",
            win_rate
        );
    }

    #[test]
    fn test_win_rate_is_high_before_threshold() {
        let store = LuckStore::new();

        let mut wins = 0u32;
        let attempts = DAILY_WIN_THRESHOLD;
        for _ in 0..attempts {
            if store.try_luck() { wins += 1; }
        }

        let win_rate = wins as f64 / attempts as f64;
        assert!(
            win_rate > 0.45,
            "Win rate before threshold ({:.2}) should be high, expected ~0.7",
            win_rate
        );
    }

    #[test]
    fn test_concurrent_access_is_safe() {
        use std::sync::Arc;
        use std::thread;

        let store = Arc::new(LuckStore::new());
        let mut handles = vec![];

        for _ in 0..10 {
            let store_clone = Arc::clone(&store);
            let handle = thread::spawn(move || {
                for _ in 0..20 {
                    store_clone.try_luck();
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().expect("Thread panicked");
        }
    }
}
