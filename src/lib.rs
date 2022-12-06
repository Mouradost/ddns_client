use std::{
    fs::{self, File},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

use colored::*;

/// Structure that holds the ip information got from [ipinfo](https://ipinfo.io)
#[derive(Serialize, Deserialize, Debug)]
pub struct IpInfo {
    ip: String,
    city: String,
    region: String,
    country: String,
    loc: String,
    org: String,
    timezone: String,
    readme: String,
}

/// Structure that holds the configuration for updating the DDNS server
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    /// Domain name
    domain: String,
    /// DDNS server token
    token: String,
    /// Request confirmation from the server
    verbose: bool,
    /// Clear all ip records from the server
    clear: bool,
}

impl Default for Config {
    /// Default config
    fn default() -> Self {
        Self {
            domain: Default::default(),
            token: Default::default(),
            verbose: true,
            clear: false,
        }
    }
}

impl Config {
    /// Get the config from config.yaml or generate a new one with default configuration
    pub fn get_config(path: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let config = match File::open(path) {
            Ok(f) => serde_yaml::from_reader(f).expect(""),
            Err(err) => {
                let config = Self::default();
                let prefix = match path.parent() {
                    Some(prefix) => Ok(prefix),
                    None => Err("Path doesn't exist!"),
                };
                fs::create_dir_all(prefix?)?;
                let f = File::create(path)?;
                serde_yaml::to_writer(f, &config)?;
                return Err(Box::new(err));
            }
        };
        Ok(config)
    }
}

/// Run gets the ip and run the update
///
/// # Errors
///
/// This function will return an error if ip provider or the DDNS server doens't respond
pub async fn run(config: &Config) -> Result<String, reqwest::Error> {
    match get_ip().await {
        Ok(ip_info) => {
            println!("{}\n{:#?}", "Got ip:".green(), ip_info);
            update_dns(config, &ip_info).await
        }
        Err(err) => Err(err),
    }
}


/// Get the ip information from https://ipinfo.io
///
/// # Errors
///
/// This function will return an error if ip checker doens't respond or the respond can't be serialize to IpInfo Structure
async fn get_ip() -> Result<IpInfo, reqwest::Error> {
    reqwest::get("https://ipinfo.io")
        .await?
        .json::<IpInfo>()
        .await
}

/// .
///
/// # Errors
///
/// This function will return an error if .
async fn update_dns(config: &Config, ip_info: &IpInfo) -> Result<String, reqwest::Error> {
    let url = format!(
        "https://www.duckdns.org/update?domains={}&token={}&ip={}&verbose={}&clear={}",
        config.domain, config.token, ip_info.ip, config.verbose, config.clear
    );
    reqwest::get(&url).await?.text().await
}
