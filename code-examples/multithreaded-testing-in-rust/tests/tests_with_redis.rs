mod common;

use common::{start_server, AppClient};
use multithreaded_testing_in_rust::{AppState, Config, ExternalApiClient};
use redis::{AsyncCommands, Client as RedisClient};
use std::sync::Arc;
use test_case::test_case;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

// Define a new Config struct specific to Redis setup
struct Config {
    redis_url: String,
    app_api_url: String,
    notifier_api_url: String,
}

async fn setup() -> (RedisClient, MockServer, AppClient) {
    // Use the new Config struct for Redis-specific settings
    let config = Config {
        redis_url: "redis://localhost:6379".to_string(),
        app_api_url: "http://localhost:9000".to_string(),
        notifier_api_url: "http://localhost:9090".to_string(),
    };

    let redis_client = RedisClient::open(&config.redis_url).unwrap();
    let mut conn = redis_client.get_async_connection().await.unwrap();
    conn.flushdb().await.unwrap(); // Clear the Redis database

    let notifier_api = ExternalApiClient {
        base_url: config.notifier_api_url.clone(),
        client: Arc::new(reqwest::Client::new()),
    };

    let app_client = AppClient {
        base_url: config.app_api_url,
        client: Arc::new(reqwest::Client::new()),
    };

    let app_state = AppState {
        db: Arc::new(conn),
        notifier_api,
    };

    start_server(app_state, Some(9000)).await;

    let mock_server = MockServer::start().await;

    (redis_client, mock_server, app_client)
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
    assert_eq!(names.first().unwrap(), "Rob");
}

#[tokio::test]
#[ignore]
async fn test_delete_name() {
    let (redis_client, mock_server, app_client) = setup().await;
    let mut conn = redis_client.get_async_connection().await.unwrap();
    conn.set("name:1", "Rob").await.unwrap();

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
