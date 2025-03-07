mod proofreader;

use std::{env, error::Error, sync::Arc};

use axum::Router;
use gcp_auth::TokenProvider;
use gemini_rs::prelude::GeminiClient;

#[derive(Clone)]
pub struct AppState {
    pub vertex_client: GeminiClient<Arc<dyn TokenProvider>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt().init();
    let api_endpoint = env::var("API_ENDPOINT")?;
    let project_id = env::var("PROJECT_ID")?;
    let location_id = env::var("LOCATION_ID")?;
    let bind_address = env::var("BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string());

    let authentication_manager = gcp_auth::provider().await?;
    tracing::info!("GCP AuthenticationManager initialized.");

    let vertex_client = GeminiClient::new(
        authentication_manager,
        api_endpoint,
        project_id,
        location_id,
    );
    let app_state = AppState { vertex_client };

    let router = Router::new().with_state(app_state);

    // Setup TCP Listener..
    let listener = tokio::net::TcpListener::bind(bind_address).await?;
    tracing::info!(addr = listener.local_addr()?.to_string(), "Server started",);
    axum::serve(listener, router).await?;

    Ok(())
}
