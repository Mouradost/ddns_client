use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to the config file such as : ~/.config/ddns_client/config.yaml
    pub path: std::path::PathBuf,

    /// Retry timeout in seconds
   #[arg(short, long, default_value_t = 600)]
   pub timeout: u64,
}
