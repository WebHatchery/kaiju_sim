//! Kaiju Server - Main entry point
//!
//! Starts the HTTP server with transfer and verification APIs.

use std::sync::Arc;

use axum::Router;
use sqlx::mysql::MySqlPoolOptions;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use kaiju_server::{
    api::{self, AppState},
    crypto::{KeyPurpose, ServerKeyPair, Signer, Verifier},
    TransferService,
    BreedingService,
    breeding_jobs::BreedingJobManager,
    ImageGenerationService,
    kaiju_repo::KaijuRepository,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,kaiju_server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting Kaiju Server...");

    // Get configuration from environment
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let server_secret_key = std::env::var("SERVER_SECRET_KEY")
        .expect("SERVER_SECRET_KEY must be set");
    let server_port: u16 = std::env::var("SERVER_PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("SERVER_PORT must be a valid port number");

    // Create database pool
    tracing::info!("Connecting to database...");
    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?;

    tracing::info!("Database connection established");

    // Run migrations
    tracing::info!("Running database migrations...");
    
    // Cleanup corrupt/duplicate migration checksums (Fix for 007 collision)
    let _ = sqlx::query("DELETE FROM _sqlx_migrations WHERE version >= 20240101000007").execute(&pool).await;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");
    tracing::info!("Migrations applied successfully");

    // Load or generate server key
    let key_pair = if server_secret_key.is_empty() || server_secret_key == "<hex_encoded_secret_key>" {
        tracing::warn!("SERVER_SECRET_KEY not configured, generating temporary key");
        tracing::warn!("Run 'cargo run --bin generate-keys' to generate a proper key");
        ServerKeyPair::generate(KeyPurpose::Ownership)
    } else {
        ServerKeyPair::from_hex(&server_secret_key, KeyPurpose::Ownership)
            .expect("Invalid SERVER_SECRET_KEY format")
    };

    let public_key_hex = key_pair.public_key_hex();
    tracing::info!("Server public key: {}", &public_key_hex[..20]);

    // Create services
    let signer = Signer::new(key_pair.secret_key);
    let transfer_service = Arc::new(TransferService::new(pool.clone(), signer));
    let breeding_service = Arc::new(BreedingService::new());
    let breeding_job_manager = Arc::new(BreedingJobManager::new());
    let image_gen_service = Arc::new(ImageGenerationService::new("assets/kaiju/generated"));
    let kaiju_repo = KaijuRepository::new(pool.clone());
    
    // Start Tournament Scheduler
    let tournament_manager = Arc::new(kaiju_server::tournament::manager::TournamentManager::new(pool.clone()));
    tokio::spawn(tournament_manager.run());

    // Create app state
    let state = Arc::new(AppState {
        db_pool: pool.clone(),
        kaiju_repo,
        transfer_service,
        breeding_service,
        breeding_job_manager,
        image_gen_service,
        verifier: Verifier::new(),
        server_public_keys: vec![public_key_hex],
    });

use tower_http::services::ServeDir; // Added import

    // Build router
    let app = Router::new()
        .merge(api::create_router(state))
        .nest_service("/assets", ServeDir::new("assets")) // Serve assets
        .layer(TraceLayer::new_for_http())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    // Start server
    let addr = format!("0.0.0.0:{}", server_port);
    tracing::info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
