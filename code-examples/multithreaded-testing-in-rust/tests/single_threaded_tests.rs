mod common;

use common::{start_server, AppClient};
use multithreaded_testing_in_rust::{AppState, Config, ExternalApiClient, Name, NameApi};
use reqwest::Client;
use sqlx::{migrate, query, PgPool};
use std::sync::Arc;
use test_case::test_case;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

async fn setup() -> (PgPool, MockServer, AppClient) {
    let config = Config {
        database_url: "postgres://user:pass@0.0.0.0:5432/test".to_string(),
        app_api_url: "http://localhost:9000".to_string(),
        notifier_api_url: "http://localhost:9090".to_string(),
    };

    let db_pool = PgPool::connect(&config.database_url).await.unwrap();
    let mut conn = db_pool.acquire().await.unwrap();

    query!("DROP TABLE IF EXISTS _sqlx_migrations;")
        .execute(conn.as_mut())
        .await
        .unwrap();
    query!("DROP TABLE IF EXISTS name;")
        .execute(conn.as_mut())
        .await
        .unwrap();
    migrate!("./migrations").run(&db_pool).await.unwrap();

    let notifier_api = ExternalApiClient {
        base_url: config.notifier_api_url.clone(),
        client: Arc::new(Client::new()),
    };

    let app_client = AppClient {
        base_url: config.app_api_url,
        client: Arc::new(Client::new()),
    };

    let app_state = AppState {
        db: db_pool.clone(),
        notifier_api,
    };

    start_server(app_state, Some(9000)).await;

    let mock_server = MockServer::start().await;

    (db_pool, mock_server, app_client)
}

fn mock_send_notification() -> Mock {
    Mock::given(method("POST"))
        .and(path("/external-api"))
        .respond_with(ResponseTemplate::new(200))
}

#[tokio::test]
#[ignore]
async fn test_create_name() {
    let (_, mock_server, app_client) = setup().await;

    mock_send_notification().mount(&mock_server).await;

    // No names should be present before creation
    let names = app_client.get_names().await;
    assert!(names.is_empty());

    // Create name
    app_client.create_name("Rob").await;

    // Only "Rob" should be fetched
    let names = app_client.get_names().await;
    assert_eq!(names.len(), 1);

    let expected_name = Name {
        id: 1,
        value: "Rob".to_string(),
    };
    assert_eq!(names.first().unwrap(), &expected_name);
}

#[test_case("Rob")]
#[test_case("Knuth")]
#[tokio::test]
async fn test_create_name_with_test_case(value: &str) {
    let (_, mock_server, app_client) = setup().await;

    mock_send_notification().mount(&mock_server).await;

    // No names should be present before creation
    let names = app_client.get_names().await;
    assert!(names.is_empty());

    // Create name
    app_client.create_name(value).await;

    // Only "Rob" should be fetched
    let names = app_client.get_names().await;
    assert_eq!(names.len(), 1);

    let expected_name = Name {
        id: 1,
        value: value.to_string(),
    };
    assert_eq!(names.first().unwrap(), &expected_name);
}

#[tokio::test]
#[ignore]
async fn test_delete_name() {
    let (db_pool, mock_server, app_client) = setup().await;
    let mut conn = db_pool.acquire().await.unwrap();
    query!("INSERT INTO name (value) VALUES ('Rob')")
        .execute(conn.as_mut())
        .await
        .unwrap();

    mock_send_notification().mount(&mock_server).await;

    // Only "Rob" should be fetched
    let names = app_client.get_names().await;
    assert_eq!(names.len(), 1);

    // Delete name
    app_client.delete_name(1).await;

    // No names should be present after deletion
    let names = app_client.get_names().await;
    assert!(names.is_empty());
}
