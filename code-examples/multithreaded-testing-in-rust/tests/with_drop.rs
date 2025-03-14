mod common;

use common::{start_server, AppClient};
use multithreaded_testing_in_rust::{AppState, Config, ExternalApiClient, Name, NameApi};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use reqwest::Client;
use sqlx::{migrate, PgPool};
use std::{sync::Arc, thread};
use test_case::test_case;
use tokio::runtime::Runtime;
use tokio_postgres::NoTls;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

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
        let db_name = self.db_name.clone();
        thread::spawn(move || {
            let future = async {
                let (client, conn) = tokio_postgres::connect(
                    "postgres://user:pass@0.0.0.0:5432",
                    tokio_postgres::NoTls,
                )
                .await
                .unwrap();

                tokio::spawn(async move {
                    if let Err(e) = conn.await {
                        eprintln!("connection error: {}", e);
                    }
                });

                client
                    .simple_query(&format!("DROP DATABASE \"{}\" WITH (FORCE)", db_name))
                    .await
                    .unwrap();
            };

            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();

            runtime.block_on(future);
        })
        .join()
        .unwrap();
    }
}

// impl Drop for DbPool {
//     fn drop(&mut self) {
//         println!("Dropping DbPool");
//         let db_name = self.db_name.clone();
//         let future = async move {
//             let (client, conn) =
//                 tokio_postgres::connect("postgres://user:pass@0.0.0.0:5432", NoTls)
//                     .await
//                     .unwrap();

//             let handle = tokio::spawn(async move {
//                 if let Err(err) = conn.await {
//                     panic!("connection error: {:?}", err);
//                 }
//             });

//             // self.inner.close();
//             client
//                 .simple_query(&format!(
//                     "DROP DATABASE IF EXISTS \"{}\" WITH (FORCE);",
//                     db_name
//                 ))
//                 .await
//                 .unwrap();

//             handle.abort();
//         };

//         std::thread::spawn(move || Runtime::new().unwrap().block_on(future))
//             .join()
//             .unwrap()
//     }
// }

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

fn mock_send_notification() -> Mock {
    Mock::given(method("POST"))
        .and(path("/external-api"))
        .respond_with(ResponseTemplate::new(200))
}

#[tokio::test]
async fn test_create_and_drop() {
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

    println!("names: {:?}", names);

    let expected_name = Name {
        id: 1,
        value: "Rob".to_string(),
    };
    assert_eq!(names.first().unwrap(), &expected_name);

    // DbPool will be automatically cleaned up when it goes out of scope
}
