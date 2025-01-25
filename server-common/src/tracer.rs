use opentelemetry_otlp::{SpanExporter, WithExportConfig as _};
use opentelemetry_sdk::{
    runtime,
    trace::{RandomIdGenerator, Sampler, TracerProvider},
    Resource,
};
use opentelemetry_semantic_conventions::resource::{SERVICE_NAME, SERVICE_VERSION};
use tracing::Span;

pub fn init_tracer(
    name: &str,
    version: &str,
    otlp_collector_endpoint: &str,
) -> Option<TracerProvider> {
    if otlp_collector_endpoint.is_empty() {
        return None;
    }

    let exporter = SpanExporter::builder()
        .with_tonic()
        .with_endpoint(otlp_collector_endpoint)
        .build()
        .expect("failed to init tracer");

    Some(
        TracerProvider::builder()
            .with_sampler(Sampler::ParentBased(Box::new(Sampler::TraceIdRatioBased(
                1.0, // TODO: configurable
            ))))
            .with_id_generator(RandomIdGenerator::default())
            .with_resource(Resource::from_schema_url(
                [
                    opentelemetry::KeyValue::new(SERVICE_NAME, name.to_string()),
                    opentelemetry::KeyValue::new(SERVICE_VERSION, version.to_string()),
                ],
                "https://opentelemetry.io/schemas/1.40.0",
            ))
            .with_batch_exporter(exporter, runtime::Tokio)
            .build(),
    )
}

pub fn current_span() -> Span {
    Span::current()
}
