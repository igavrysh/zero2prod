use std::net::TcpListener;
use sqlx::postgres::PgPoolOptions;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{EnvFilter, Registry, prelude::__tracing_subscriber_SubscriberExt};
use zero2prod::{startup::run, configuration::get_configuration, email_client::EmailClient};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    LogTracer::init().expect("Failed to set logger");

    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    let formatting_layer
        = BunyanFormattingLayer::new("zero2prod".into(), std::io::stdout);

    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);

    set_global_default(subscriber).expect("Failed to set subscriber");

    let configuration = get_configuration()
        .expect("Failed to read configuration");

    let connection_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.database.with_db());

    let sender_email = configuration.email_client.sender()
        .expect("Invalid sender email address.");
    let email_client = EmailClient::new(
        configuration.email_client.base_url,
        sender_email,
        configuration.email_client.authorization_token
    );

    let address = format!("{}:{}", 
        configuration.application.host, configuration.application.port
    );

    let listener = TcpListener::bind(address)?;

    run(listener, connection_pool, email_client)?.await
}
