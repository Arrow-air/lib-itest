use crate::job;
use async_trait::async_trait;
use hyper::{Body, Client, Method, Request};
use serde_json;
use svc_assets_client_rest::types::*;

fn get_server_addr() -> Result<String, std::env::VarError> {
    let host = std::env::var("ASSETS_HOST_REST")?;
    let port = std::env::var("ASSETS_PORT_REST")?;

    Ok(format!("{host}:{port}"))
}

//=====================================================================
// Aircraft Steps
//=====================================================================

#[derive(Clone)]
pub struct CreateAircraftStep {
    id: String,
    name: String,
    owner: String,
    model: String,
    manufacturer: String,
    icao: String,
}

#[async_trait]
impl job::Step for CreateAircraftStep {
    fn description(&self) -> String {
        format!(
            "Create an aircraft with the following metadata:
                Id:     {}
                Name:   {}
                Owner:  {}
                Model:  {}
                ICAO:   {}
                Mfr:    {}
            ",
            self.id, self.name, self.model, self.owner, self.icao, self.manufacturer,
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

        let endpoint = format!("{url}/assets/vehicles");

        //
        // Build Request
        //
        let data = RegisterAircraftPayload {
            manufacturer: self.manufacturer.clone(),
            model: self.model.clone(),
            max_payload_kg: 1000.0,
            max_range_km: 1000.0,
            owner: self.owner.clone(),
            registration_number: self.id.clone(),
            serial_number: self.icao.clone(), // TODO Type ID
            status: AssetStatus::Available,
            whitelist: vec![],
            description: Some(self.name.clone()),
            group_id: None,
            last_maintenance: None,
            name: Some(self.name.clone()),
            next_maintenance: None,
        };

        let data_str = serde_json::to_string(&data).unwrap();
        let request = Request::builder()
            .method(Method::POST)
            .uri(endpoint.clone())
            .header("content-type", "application/json")
            .body(Body::from(data_str));

        let Ok(request) = request else {
            return Err(
                request.unwrap_err().to_string()
            )
        };

        //
        // Send Request
        //
        let result = client.request(request).await;
        let Ok(_) = result else {
            return Err(
                result.unwrap_err().to_string()
            );
        };

        Ok(())
    }
}


//=====================================================================
// Vertiport Steps
//=====================================================================

#[derive(Clone)]
pub struct CreateVertiportStep {
    name: String,
    owner: String,
    latitude: f64,
    longitude: f64
}

#[async_trait]
impl job::Step for CreateVertiportStep {
    fn description(&self) -> String {
        format!(
            "Create a vertiport with the following metadata:
                Name:      {}
                Owner:     {}
                Latitude:  {}
                Longitude: {}
            ",
            self.name,
            self.owner,
            self.latitude,
            self.longitude
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

        let endpoint = format!("{url}/assets/vertiports");

        //
        // Build Request
        //
        let data = RegisterVertiportPayload {
            name: Some(self.name.clone()),
            group_id: None,
            owner: self.owner.clone(),
            whitelist: vec![],
            status: AssetStatus::Available,
            description: Some(self.name.clone()), // TODO
            location: Location {
                latitude: self.latitude.into(),
                longitude: self.longitude.into()
            },
        };

        let data_str = serde_json::to_string(&data).unwrap();
        let request = Request::builder()
            .method(Method::POST)
            .uri(endpoint.clone())
            .header("content-type", "application/json")
            .body(Body::from(data_str));

        let Ok(request) = request else {
            return Err(
                request.unwrap_err().to_string()
            )
        };

        //
        // Send Request
        //
        let result = client.request(request).await;
        let Ok(_) = result else {
            return Err(
                result.unwrap_err().to_string()
            );
        };

        Ok(())
    }
}

