use opentelemetry_otlp::{SpanExporter, WithExportConfig as _};
use opentelemetry_sdk::{
    runtime,
    trace::{RandomIdGenerator, Sampler, TracerProvider},
    Resource,
};
use opentelemetry_semantic_conventions::resource::{SERVICE_NAME, SERVICE_VERSION};

pub fn init_tracer(name: String, version: String) -> TracerProvider {
    let exporter = SpanExporter::builder()
        .with_tonic()
        .with_endpoint("http://localhost:4317") // TODO: configurable
        .build()
        .expect("failed to init tracer");

    TracerProvider::builder()
        .with_sampler(Sampler::ParentBased(Box::new(Sampler::TraceIdRatioBased(
            1.0, // TODO: configurable
        ))))
        .with_id_generator(RandomIdGenerator::default())
        .with_resource(Resource::from_schema_url(
            [
                opentelemetry::KeyValue::new(SERVICE_NAME, name),
                opentelemetry::KeyValue::new(SERVICE_VERSION, version),
            ],
            "https://opentelemetry.io/schemas/1.20.0",
        ))
        .with_batch_exporter(exporter, runtime::Tokio)
        .build()
}
