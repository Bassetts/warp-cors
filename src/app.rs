use std::net::SocketAddr;

use clap::{crate_authors, crate_name, crate_version, App, AppSettings, Arg};
use log::{debug, info};

use crate::routes;

pub struct Config {
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn parse() -> Result<Config, Box<dyn std::error::Error>> {
        let matches = App::new(crate_name!())
            .author(crate_authors!())
            .version(crate_version!())
            .setting(AppSettings::UnifiedHelpMessage)
            .arg(
                Arg::with_name("port")
                    .short("p")
                    .long("port")
                    .value_name("PORT"),
            )
            .arg(
                Arg::with_name("host")
                    .short("h")
                    .long("hostname")
                    .value_name("HOST"),
            )
            .get_matches();

        let port = matches.value_of("port").map_or(Ok(3030), |p| p.parse())?;

        let host = matches
            .value_of("host")
            .unwrap_or(env!("CARGO_PKG_NAME"))
            .to_owned();

        Ok(Config { port, host })
    }
}

pub async fn run(config: Config) {
    debug!("Using port: {}", config.port);

    let addr: SocketAddr = ([0, 0, 0, 0], config.port).into();

    debug!("Using host/alias: {}", config.host);

    info!("Listening on {}", addr);
    warp::serve(routes::routes(config.host)).run(addr).await;
}
