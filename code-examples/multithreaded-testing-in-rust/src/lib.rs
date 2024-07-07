#![allow(dead_code)]

use std::sync::Arc;

use async_trait::async_trait;
use axum::{
    extract::{FromRef, Path, State},
    http::StatusCode,
    routing::{delete, get, post},
    Json, Router,
};
use reqwest::{Body, Client};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub notifier_api: ExternalApiClient,
}

impl FromRef<AppState> for PgPool {
    fn from_ref(state: &AppState) -> Self {
        state.db.clone()
    }
}

pub struct Config {
    pub database_url: String,
    pub app_api_url: String,
    pub notifier_api_url: String,
}

pub async fn router(app_state: AppState) -> Router {
    Router::new()
        .route("/names", get(get_names))
        .route("/names", post(post_name))
        .route("/names/:id", get(get_name))
        .route("/names/:id", delete(delete_name))
        .with_state(app_state)
}

#[async_trait]
pub trait NameApi {
    async fn get_names(&self) -> Vec<Name>;
    async fn get_name(&self, id: i32) -> Name;
    async fn create_name(&self, name: &str);
    async fn delete_name(&self, id: i32);
}

pub async fn get_names(State(state): State<AppState>) -> Json<Vec<Name>> {
    let names = state.db.find_all().await.unwrap();
    Json(names)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostNameRequest {
    pub value: String,
}

async fn post_name(
    State(state): State<AppState>,
    Json(payload): Json<PostNameRequest>,
) -> StatusCode {
    state.db.create(&payload.value).await.unwrap();
    StatusCode::CREATED
}

async fn get_name(State(state): State<AppState>, Path(path): Path<i32>) -> Json<Name> {
    let name = state.db.find(path).await.unwrap();
    Json(name)
}

async fn delete_name(State(state): State<AppState>, Path(path): Path<i32>) -> StatusCode {
    state.db.delete(path).await.unwrap();
    StatusCode::OK
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Name {
    pub id: i32,
    pub value: String,
}

#[async_trait]
pub trait NameStore {
    async fn find_all(&self) -> Result<Vec<Name>, sqlx::Error>;
    async fn find(&self, id: i32) -> Result<Name, sqlx::Error>;
    async fn create(&self, name: &str) -> Result<(), sqlx::Error>;
    async fn delete(&self, id: i32) -> Result<(), sqlx::Error>;
}

#[async_trait]
impl NameStore for PgPool {
    async fn find_all(&self) -> Result<Vec<Name>, sqlx::Error> {
        let mut conn = self.acquire().await?;
        sqlx::query_as!(Name, "SELECT * FROM name")
            .fetch_all(conn.as_mut())
            .await
    }

    async fn find(&self, id: i32) -> Result<Name, sqlx::Error> {
        let mut conn = self.acquire().await?;
        sqlx::query_as!(Name, "SELECT * FROM name WHERE id = $1", id)
            .fetch_one(conn.as_mut())
            .await
    }

    async fn create(&self, name: &str) -> Result<(), sqlx::Error> {
        let mut conn = self.acquire().await?;
        sqlx::query_as!(Name, "INSERT INTO name (value) VALUES ($1)", name)
            .execute(conn.as_mut())
            .await
            .map(|_| ())
    }

    async fn delete(&self, id: i32) -> Result<(), sqlx::Error> {
        let mut conn = self.acquire().await?;
        sqlx::query!("DELETE FROM name WHERE id = $1", id)
            .execute(conn.as_mut())
            .await
            .map(|_| ())
    }
}

#[derive(Clone)]
pub struct ExternalApiClient {
    pub base_url: String,
    pub client: Arc<Client>,
}

#[async_trait]
pub trait NotifierApi {
    async fn send_notification(&self, name: Name);
}

impl From<Name> for Body {
    fn from(val: Name) -> Self {
        serde_json::to_string(&val).unwrap().into()
    }
}

#[async_trait]
impl NotifierApi for ExternalApiClient {
    async fn send_notification(&self, name: Name) {
        let url = format!("{}/external-api", self.base_url);
        self.client.post(&url).body(name).send().await.unwrap();
    }
}
