mod proofreader;
mod routes;

use std::{env, error::Error, sync::Arc};

use axum::{Router, http::Method, routing::post};
use gcp_auth::TokenProvider;
use gemini_rs::prelude::GeminiClient;
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    services::ServeDir,
};

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

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any);

    // Sets up a compression layer that supports brotli, deflate, gzip, and zstd.
    let compression_layer = CompressionLayer::new()
        .br(true)
        .deflate(true)
        .gzip(true)
        .zstd(true);

    let router = Router::new()
        .fallback_service(ServeDir::new("static"))
        .route("/proofread", post(routes::proofread))
        .layer(cors)
        .layer(compression_layer)
        .with_state(app_state);

    // Setup TCP Listener..
    let listener = tokio::net::TcpListener::bind(bind_address).await?;
    tracing::info!(addr = listener.local_addr()?.to_string(), "Server started",);
    axum::serve(listener, router).await?;

    Ok(())
}
