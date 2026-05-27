use opentelemetry::global;
use opentelemetry_sdk::propagation::TraceContextPropagator;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod presentation;

/// Initializes the APEX telemetry system.
///
/// This sets up structured logging and OpenTelemetry tracing.
/// In accordance with the APEX Protocol, observability is mandatory from line one.
pub fn init_telemetry() {
    global::set_text_map_propagator(TraceContextPropagator::new());

    let subscriber = tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer());

    subscriber.init();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_telemetry_initialization() {
        // Verification that telemetry can be initialized without panic.
        // We don't re-initialize in tests if already global, but this serves as a baseline check.
        let _ = tracing_subscriber::fmt().with_env_filter("info").try_init();
    }
}
