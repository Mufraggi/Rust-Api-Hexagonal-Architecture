use actix_web::dev::ServiceRequest;
use actix_web_opentelemetry::RequestMetrics;
use futures::SinkExt;
use opentelemetry::global;
use opentelemetry::sdk::propagation::TraceContextPropagator;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Registry;


pub struct TelemetryClient {
  //  pub request_metrics: RequestMetrics<dyn Send + Clone>
}

impl TelemetryClient {
    pub fn init()  {
        global::set_text_map_propagator(TraceContextPropagator::new());
        let tracer = opentelemetry_jaeger::new_pipeline()
            .with_service_name("users_app")
            .install_simple()
            .unwrap();

        Registry::default()
            .with(tracing_subscriber::EnvFilter::new("INFO"))
            .with(tracing_subscriber::fmt::layer())
            .with(tracing_opentelemetry::layer().with_tracer(tracer))
            .init();
    }
}
