use std::{thread, time::Duration};

use clap::Parser;
use colored::*;

use args::Cli;
use ddns_client::{run, Config};

mod args;

/// A simple CLI DDNS client updater
/// 
/// # Steps
///
/// - Parse user arguments
/// - Call run
/// - Retry after X seconds if the update was not successful
#[tokio::main]
async fn main() {
    let args = Cli::parse();
    match Config::get_config(&args.path){
        Ok(config) => {
                println!("{}\n{:#?}", "Found config:".green(), config);
                let mut updated = false;
                while !updated {
                    updated = match run(&config).await {
                        Ok(status) => {
                            println!("{}\n{}", "DDNS status:".green(), status.purple());
                            status.starts_with("OK")
                        },
                        Err(err) => {
                            eprint!("{}\n  {}\n", "Error:".red(), err);
                            false
                        },
                    };

                    if !updated {
                        let msg = format!("Retrying after {}s...", args.timeout);
                        println!("{}", msg.yellow());
                        thread::sleep(Duration::from_secs(args.timeout));
                    }
                }

                println!("{}", "DDNS updated!".green());
        },
        Err(err) => eprint!("{}\n  {}\n  {}", "Config error:".red(), err, "A new sample config has been generated!\n  Please fill the missing informations and try again...".yellow()),
    }
}
