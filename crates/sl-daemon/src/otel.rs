//! Feature-gated OTLP trace export.

use std::error::Error;

use opentelemetry::trace::TracerProvider as _;
use opentelemetry_otlp::WithExportConfig as _;
use opentelemetry_sdk::trace::SdkTracerProvider;
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

/// Install the normal formatting layer plus an OTLP tracing layer.
///
/// The provider is returned so the caller can keep it alive and flush it
/// during a graceful shutdown.
pub(crate) fn init(filter: EnvFilter, endpoint: &str) -> Result<SdkTracerProvider, Box<dyn Error>> {
    let exporter =
        opentelemetry_otlp::SpanExporter::builder().with_tonic().with_endpoint(endpoint).build()?;
    let provider = SdkTracerProvider::builder().with_batch_exporter(exporter).build();
    let tracer = provider.tracer("sl-daemon");

    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer().with_target(true))
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .try_init()?;

    Ok(provider)
}
