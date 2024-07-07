mod common;

use std::sync::Arc;

use common::{mock_send_notification, start_server, AppClient};
use multithreaded_testing_in_rust::{AppState, ExternalApiClient, Name, NameApi};
use reqwest::Client;
use sqlx::{query, PgPool};
use wiremock::MockServer;

async fn setup(db_pool: PgPool) -> (PgPool, MockServer, AppClient) {
    let mock_server = MockServer::start().await;

    let notifier_api = ExternalApiClient {
        base_url: mock_server.uri(),
        client: Arc::new(Client::new()),
    };

    let app_state = AppState {
        db: db_pool.clone(),
        notifier_api,
    };

    let port = start_server(app_state, None).await;

    let app_client = AppClient {
        base_url: format!("http://localhost:{port}"),
        client: Arc::new(Client::new()),
    };

    (db_pool, mock_server, app_client)
}

#[sqlx::test]
async fn test_create_name(pool: PgPool) {
    let (_, mock_server, app_client) = setup(pool).await;

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

// #[test_case("Rob")]
// #[sqlx::test]
// async fn test_create_name_with_test_case(pool: PgPool, value: &str) {
//     let (_, mock_server, app_client) = setup(pool).await;
//
//     mock_send_notification().mount(&mock_server).await;
//
//     // No names should be present before creation
//     let names = app_client.get_names().await;
//     assert!(names.is_empty());
//
//     // Create name
//     app_client.create_name(value).await;
//
//     // Only "Rob" should be fetched
//     let names = app_client.get_names().await;
//     assert_eq!(names.len(), 1);
//
//     let expected_name = Name {
//         id: 1,
//         value: value.to_string(),
//     };
//     assert_eq!(names.first().unwrap(), &expected_name);
// }

#[sqlx::test]
async fn test_delete_name(pool: PgPool) {
    let (db_pool, mock_server, app_client) = setup(pool).await;
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
