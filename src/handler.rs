use std::sync::Arc;

use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    Json,
};
use mysql::serde_json::json;

use crate::{
    database::{SigninRequest, SignupRequest},
    AppState,
};

pub async fn create_table(State(shared_state): State<Arc<AppState>>) -> (StatusCode, Json<String>) {
    shared_state.db.create_table().await;
    (StatusCode::OK, Json("table created".to_string()))
}

pub async fn signup(
    State(shared_state): State<Arc<AppState>>,
    Json(request): Json<SignupRequest>,
) -> (StatusCode, Json<String>) {
    if shared_state.db.user_exists(request.username.clone()).await {
        (
            StatusCode::CONFLICT,
            Json("user already exists".to_string()),
        )
    } else {
        let headers = HeaderMap::new();
        let body = json!({
            "username": request.username.clone(),
            "first_name": request.first_name,
            "last_name": request.last_name,
            "gender": request.gender,
            "dob": request.dob
        });

        let client = reqwest::Client::new();
        let resp = client
            .post("http://localhost:8080/user/add")
            .headers(headers)
            .json(&body)
            .send()
            .await
            .unwrap();
        println!("mongo response: {:?}", resp);

        shared_state.db.add_user(request).await;
        (StatusCode::OK, Json("signup successful".to_string()))
    }
}

pub async fn signin(
    State(shared_state): State<Arc<AppState>>,
    Json(req): Json<SigninRequest>,
) -> (StatusCode, Json<String>) {
    if shared_state.db.user_exists(req.username.clone()).await {
        if shared_state.db.authorize_user(req).await {
            (StatusCode::OK, Json("login successful".to_string()))
        } else {
            (StatusCode::OK, Json("login failed".to_string()))
        }
    } else {
        (StatusCode::OK, Json("user does not exist".to_string()))
    }
}
