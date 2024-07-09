mod common;

use std::sync::Arc;

use common::{mock_send_notification, start_server, AppClient};
use multithreaded_testing_in_rust::{AppState, ExternalApiClient, Name, NameApi};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use reqwest::Client;
use sqlx::{migrate, query, PgPool};
use test_case::test_case;
use tokio_postgres::NoTls;
use wiremock::MockServer;

#[derive(Clone)]
struct DbPool {
    inner: PgPool,
    db_name: String,
}

impl DbPool {
    async fn spinup() -> Self {
        let mut rng = thread_rng();
        let db_name: String = (&mut rng)
            .sample_iter(Alphanumeric)
            .take(8)
            .map(char::from)
            .collect();

        let (client, conn) = tokio_postgres::connect("postgres://user:pass@0.0.0.0:5432", NoTls)
            .await
            .unwrap();

        tokio::spawn(async move {
            if let Err(e) = conn.await {
                eprintln!("connection error: {}", e);
            }
        });

        client
            .simple_query(&format!("CREATE DATABASE \"{db_name}\";"))
            .await
            .unwrap();

        let db_pool = PgPool::connect(&format!("postgres://user:pass@0.0.0.0:5432/{db_name}"))
            .await
            .unwrap();

        Self {
            inner: db_pool,
            db_name,
        }
    }
}

impl Drop for DbPool {
    fn drop(&mut self) {
        futures::executor::block_on(async move {
            println!("Dropping database {}...", self.db_name);

            let (client, conn) =
                tokio_postgres::connect("postgres://user:pass@0.0.0.0:5432", NoTls)
                    .await
                    .unwrap();

            println!("Connection acquired...");

            let guard = tokio::spawn(async move {
                if let Err(e) = conn.await {
                    eprintln!("connection error: {}", e);
                }
            });

            self.inner.close().await;
            client
                .simple_query(&format!("DROP DATABASE \"{}\" WITH (FORCE)", self.db_name))
                .await
                .unwrap();
            guard.abort()
        });
    }
}

async fn setup() -> (DbPool, MockServer, AppClient) {
    let mock_server = MockServer::start().await;
    let notifier_api = ExternalApiClient {
        base_url: mock_server.uri(),
        client: Arc::new(Client::new()),
    };

    let db_pool = DbPool::spinup().await;
    migrate!("./migrations").run(&db_pool.inner).await.unwrap();

    let app_state = AppState {
        db: db_pool.inner.clone(),
        notifier_api,
    };

    let port = start_server(app_state, None).await;

    let app_client = AppClient {
        base_url: format!("http://localhost:{port}"),
        client: Arc::new(Client::new()),
    };

    (db_pool, mock_server, app_client)
}

#[tokio::test(flavor = "multi_thread")]
async fn test_create_name() {
    let (db_pool, mock_server, app_client) = setup().await;

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

    // db_pool.teardown().await;
}

#[test_case("Rob")]
#[test_case("Knuth")]
#[tokio::test(flavor = "multi_thread")]
async fn test_create_name_with_test_case(value: &str) {
    let (db_pool, mock_server, app_client) = setup().await;

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

    // db_pool.teardown().await;
}

#[tokio::test(flavor = "multi_thread")]
async fn test_delete_name() {
    let (db_pool, mock_server, app_client) = setup().await;
    let mut conn = db_pool.inner.acquire().await.unwrap();
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

    // db_pool.teardown().await;
}
