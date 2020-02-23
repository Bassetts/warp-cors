use std::process;

mod app;
mod client;
mod error;
mod filters;
mod handlers;
mod routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let config = app::Config::parse().unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    app::run(config).await;

    Ok(())
}
