use std::{net::SocketAddr, sync::Arc};

use async_trait::async_trait;
use multithreaded_testing_in_rust::{router, AppState, Name, NameApi, PostNameRequest};
use reqwest::Client;
use tokio::{net::TcpListener, sync::oneshot};
use wiremock::{
    matchers::{method, path},
    Mock,
};

#[derive(Clone)]
pub struct AppClient {
    pub base_url: String,
    pub client: Arc<Client>,
}

#[async_trait]
impl NameApi for AppClient {
    async fn get_names(&self) -> Vec<Name> {
        let response = self
            .client
            .get(&format!("{}/names", self.base_url))
            .send()
            .await
            .unwrap();

        response.json::<Vec<Name>>().await.unwrap()
    }

    async fn get_name(&self, id: i32) -> Name {
        let response = self
            .client
            .get(&format!("{}/names/{}", self.base_url, id))
            .send()
            .await
            .unwrap();

        response.json::<Name>().await.unwrap()
    }

    async fn create_name(&self, name: &str) {
        self.client
            .post(&format!("{}/names", self.base_url))
            .json(&PostNameRequest {
                value: name.to_string(),
            })
            .send()
            .await
            .unwrap();
    }

    async fn delete_name(&self, id: i32) {
        self.client
            .delete(&format!("{}/names/{}", self.base_url, id))
            .send()
            .await
            .unwrap();
    }
}

pub fn mock_send_notification() -> Mock {
    Mock::given(method("POST"))
        .and(path("/external-api"))
        .respond_with(wiremock::ResponseTemplate::new(200))
}

pub async fn start_server(app_state: AppState, port: Option<u16>) -> u16 {
    let (set_dynamic_port, dynamic_port) = oneshot::channel::<u16>();

    tokio::spawn(async move {
        let router = router(app_state).await;
        let dynamic_addr = SocketAddr::from(([0, 0, 0, 0], port.unwrap_or(0)));
        let listener = TcpListener::bind(dynamic_addr).await.unwrap();
        set_dynamic_port
            .send(listener.local_addr().unwrap().port())
            .unwrap();

        axum::serve(listener, router).await.unwrap();
    });

    dynamic_port.await.unwrap()
}
