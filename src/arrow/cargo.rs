use crate::job;
use async_trait::async_trait;
use hyper::{Body, Client, Method, Request};
use serde_json;
use svc_cargo_client_rest::types::*;

fn get_server_addr() -> Result<String, std::env::VarError> {
    let host = std::env::var("CARGO_HOST_REST")?;
    let port = std::env::var("CARGO_PORT_REST")?;

    Ok(format!("{host}:{port}"))
}

#[derive(Clone)]
struct QueryVertiportsStep {
    latitude: f32,
    longitude: f32,
    radius_km: f32,
}

#[async_trait]
impl job::Step for QueryVertiportsStep {
    fn description(&self) -> String {
        format!(
            "Request a list of vertiports within {}
        kilometers of coordinates ({}, {}).",
            self.radius_km, self.latitude, self.longitude
        )
    }

    async fn action(&self) -> job::StepResult {
        let client = Client::builder()
            .pool_idle_timeout(std::time::Duration::from_secs(10))
            .build_http();

        let Ok(url) = get_server_addr() else {
            return Err(
                "Failed to build endpoint URL.".into()
            );
        };

        let endpoint = format!("{url}/vertiports");

        //
        // Build Request
        //
        let data = VertiportsQuery {
            latitude: self.latitude,
            longitude: self.longitude,
        };

        let result = Request::builder()
            .method(Method::POST)
            .uri(endpoint)
            .header("content-type", "application/json")
            .body(Body::from(serde_json::to_string(&data).unwrap()));
        let Ok(req) = result else {
            return Err(
                format!("Failed to build request: {:?}", result.err())
            );
        };

        let result = client.request(req).await;
        let Ok(_) = result else {
            return Err(
                format!("Request failed: {:?}", result.err())
            );
        };

        Ok(())
    }
}
