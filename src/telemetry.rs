use std::time::Duration;

use opentelemetry_sdk::trace::SdkTracerProvider;

const DEFAULT_OTLP_ENDPOINT: &str = "http://monitoring.internal.rubas.dev:4317";

pub fn init_tracing(service_name: &'static str) -> Option<SdkTracerProvider> {
    use opentelemetry::trace::TracerProvider;
    use opentelemetry_otlp::{SpanExporter, WithExportConfig};
    use opentelemetry_sdk::{Resource, trace as sdktrace};
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

    let endpoint = std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT")
        .unwrap_or_else(|_| DEFAULT_OTLP_ENDPOINT.to_string());

    let exporter = SpanExporter::builder()
        .with_tonic()
        .with_endpoint(endpoint)
        .with_timeout(Duration::from_secs(3))
        .build()
        .ok()?;

    let provider = sdktrace::SdkTracerProvider::builder()
        .with_batch_exporter(exporter)
        .with_resource({
            let project = std::env::current_dir()
                .ok()
                .and_then(|path| {
                    let components: Vec<_> = path.components().collect();
                    let len = components.len();
                    (len >= 2).then(|| {
                        format!(
                            "{}/{}",
                            components[len - 2].as_os_str().to_string_lossy(),
                            components[len - 1].as_os_str().to_string_lossy()
                        )
                    })
                })
                .unwrap_or_default();
            let hostname = std::env::var("HOSTNAME")
                .or_else(|_| {
                    std::fs::read_to_string("/etc/hostname").map(|value| value.trim().to_string())
                })
                .unwrap_or_else(|_| "unknown".into());

            Resource::builder_empty()
                .with_service_name(service_name)
                .with_attributes([
                    opentelemetry::KeyValue::new("project", project),
                    opentelemetry::KeyValue::new("host.name", hostname),
                ])
                .build()
        })
        .build();

    let tracer = provider.tracer(service_name);
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    tracing_subscriber::registry()
        .with(telemetry)
        .try_init()
        .ok()?;

    Some(provider)
}

pub fn shutdown_tracing(provider: Option<SdkTracerProvider>) {
    if let Some(provider) = provider {
        let _ = provider.shutdown();
    }
}
