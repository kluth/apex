use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

/// Initializes the telemetry and logging infrastructure.
/// This establishes the 'Ecosystem' standard for observability in APEX.
pub fn init_telemetry() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("APEX Telemetry Initialized.");
}

/// Shuts down the telemetry provider gracefully.
pub fn shutdown_telemetry() {
    // Fmt subscriber doesn't require explicit shutdown
}
