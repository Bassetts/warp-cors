use std::net::SocketAddr;

use clap::{Arg, ArgAction, Command};
use log::{debug, error, info};
#[cfg(unix)]
use tokio::signal::unix::{signal, SignalKind};

use crate::error;
use crate::routes;

pub struct Config {
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn parse() -> Result<Config, Box<dyn std::error::Error>> {
        let matches = Command::new(env!("CARGO_PKG_NAME"))
            .author(env!("CARGO_PKG_AUTHORS"))
            .version(env!("CARGO_PKG_VERSION"))
            .next_line_help(false)
            .disable_help_flag(true)
            .arg(Arg::new("port").short('p').long("port").value_name("PORT"))
            .arg(
                Arg::new("host")
                    .short('h')
                    .long("hostname")
                    .value_name("HOST"),
            )
            .arg(
                Arg::new("help")
                    .long("help")
                    .action(ArgAction::HelpLong)
                    .help("Print help"),
            )
            .get_matches();

        let port = matches
            .get_one::<String>("port")
            .map_or(Ok(3030), |p| p.parse())?;

        let host = matches
            .get_one::<String>("host")
            .map(String::as_str)
            .unwrap_or(env!("CARGO_PKG_NAME"))
            .to_owned();

        Ok(Config { port, host })
    }
}

pub async fn run(config: Config) {
    debug!("Using port: {}", config.port);

    let addr: SocketAddr = ([0, 0, 0, 0], config.port).into();

    debug!("Using host/alias: {}", config.host);

    let server = warp::serve(routes::routes(config.host)).bind(addr).await;
    info!("Listening on http://{}", addr);

    server
        .graceful(async {
            let signal = shutdown_signal().await;
            if let Err(e) = signal {
                error!("server error: {}", e);
            }
        })
        .run()
        .await;
}

#[cfg(not(unix))]
async fn shutdown_signal() -> Result<(), error::Error> {
    tokio::signal::ctrl_c().await?;

    info!("Received CTRL-C event");

    Ok(())
}

#[cfg(unix)]
async fn shutdown_signal() -> Result<(), error::Error> {
    let mut term = signal(SignalKind::terminate())?;
    let mut int = signal(SignalKind::interrupt())?;

    tokio::select! {
        _ = int.recv() => {
            info!("Received SIGINT");
        }
        _ = term.recv() => {
            info!("Received SIGTERM");
        }
    }

    Ok(())
}
