mod config;
mod error;
mod logging;
mod utils;
mod websocket;

use std::process::ExitCode;
use std::time::Duration;
use std::thread::sleep;
use std::env::var;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use clap::{value_parser, ValueEnum, Arg, ArgAction, Command};
use log::{error, info};
use tungstenite::Message;

use config::Config;
use error::AppError;
use logging::setup_logging;
use websocket::WebSocketClient;

mod build_date {
    include!(concat!(env!("OUT_DIR"), "/build_date.rs"));
}

fn run(shutdown: Arc<AtomicBool>) -> Result<(), AppError> {
    // Load configuration from environment
    let config = Config::from_env()?;

    // Initialize WebSocket connection
    let mut client = WebSocketClient::new(config.clone())?;

    // Log the configuration info
    info!("{}", client.get_config_info());

    // Send an initial ping to verify connection
    client.send_ping_with_retry(3)?;

    let mut count = 0;

    // Main processing loop
    loop {
        // Check for shutdown signal
        if shutdown.load(Ordering::Relaxed) {
            info!("Shutdown signal received, closing gracefully");
            println!("Shutdown signal received, closing gracefully");
            break;
        }
        match client.read_message() {
            Ok(msg) => {
                if !msg.is_empty() {
                    info!("Received msg: {}", msg);
                    println!("Received message containing {:?} bytes", msg.len());
                    
                    // Send a pong response if we received a ping
                    if msg.is_ping() {
                        info!("Received ping, sending pong");
                        if let Err(e) = client.write_message(Message::Pong(vec![].into())) {
                            error!("Failed to send pong: {}", e);
                            // Try to reconnect if sending fails
                            if let Err(e) = client.reconnect() {
                                error!("Failed to reconnect: {}", e);
                                break;
                            }
                        }
                    }
                } else {
                    println!("Received empty message");
                }
            },
            Err(e) => {
                error!("Error reading message: {}", e);
                // Try to reconnect on error
                if let Err(reconnect_err) = client.reconnect() {
                    error!("Failed to reconnect: {}", reconnect_err);
                    break;
                }
                info!("Reconnected successfully");
            }
        }
        
        count += 1;
        if count >= config.epoch_count {
            println!("Power.Trade websocket client closing after {} epochs exceeded", count);
            info!("Power.Trade websocket client closing after {} epochs exceeded", count);
            break;
        } else {
            println!("Power.Trade websocket client sleeping for {} secs on iteration {} of {}", 
                     config.sleep_duration, count, config.epoch_count);
        }
        
        sleep(Duration::from_secs(config.sleep_duration));
    }
    
    Ok(())
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Environment {
    Development,
    Test,
    Production,
}

fn main() -> ExitCode {
    // Use the function from the included module
    let build_date = build_date::build_date();
    
    // Create a static version string that can be used with clap
    //let version_str = format!("version {} built on {}", env!("CARGO_PKG_VERSION"), build_date);
    let version_string = format!("version {} built on {}", env!("CARGO_PKG_VERSION"), build_date);
    let static_version: &'static str = Box::leak(version_string.into_boxed_str());

    // Parse command line arguments
    let matches = Command::new("Power.Trade Websocket Client")
        .version(static_version)  // Use as_str() to get a &str from String
        .about("Client for Power.Trade WebSocket API")
        .arg(
            Arg::new("env")
                .action(ArgAction::Set)
                .alias("environment")
                .short('e')
                .long("env")
                .required(true)
                .help("Select environment for the WS Client to run against")
                .value_name("pt_env")
                .value_parser(value_parser!(Environment))
        )
        .arg(
            Arg::new("log-level")
                .long("log-level")
                .help("Set logging level (error, warn, info, debug, trace)")
                .default_value("info")
        )
        .arg(
            Arg::new("log-file")
                .long("log-file")
                .help("Path to log file")
                .default_value("logs/app.log")
        )
        .get_matches();
    
    // Use the same version string for logging
    println!("Starting websocket client for power.trade [{}]", static_version);
    info!("Starting websocket client for power.trade [{}]", static_version);
    
    // Get environment and load appropriate .env file
    let pt_env = matches.get_one::<Environment>("env").expect("env is required");

    let env_file = match pt_env {
        Environment::Development => {
            println!("Environment is set to DEV");
            ".env.dev"
        },
        Environment::Test => {
            println!("Environment is set to TEST");
            ".env.test"
        },
        Environment::Production => {
            println!("Environment is set to PROD");
            ".env.prod"
        },
    };

    if let Err(e) = dotenvy::from_filename(env_file) {
        eprintln!("Failed to load environment file '{}': {}", env_file, e);
        eprintln!("Please ensure the file exists and is readable.");
        return ExitCode::FAILURE;
    }
    
    // Setup logging
    let log_level = matches.get_one::<String>("log-level").unwrap();
    let log_file = matches.get_one::<String>("log-file").unwrap();
    
    let level = match log_level.to_lowercase().as_str() {
        "error" => log::LevelFilter::Error,
        "warn" => log::LevelFilter::Warn,
        "info" => log::LevelFilter::Info,
        "debug" => log::LevelFilter::Debug,
        "trace" => log::LevelFilter::Trace,
        _ => log::LevelFilter::Info,
    };
    
    if let Err(e) = setup_logging(log_file, level) {
        eprintln!("Failed to initialize logging: {}", e);
        return ExitCode::FAILURE;
    }

    // Setup graceful shutdown handler
    let shutdown = Arc::new(AtomicBool::new(false));
    let shutdown_clone = shutdown.clone();

    ctrlc::set_handler(move || {
        info!("Received interrupt signal (Ctrl+C)");
        println!("\nReceived interrupt signal, shutting down gracefully...");
        shutdown_clone.store(true, Ordering::Relaxed);
    }).expect("Error setting Ctrl-C handler");

    // Run with retry logic
    let max_retries = var("PT_MAX_RETRIES").unwrap_or_else(|_| "5".to_string())
        .parse::<u32>().unwrap_or(5);

    let mut retries = max_retries;
    loop {
        match run(shutdown.clone()) {
            Ok(()) => break,
            Err(e) => {
                error!("Error: {}", e);
                if retries > 0 {
                    retries -= 1;
                    info!("Retrying... attempts left: {}", retries);
                    sleep(Duration::from_secs(5));
                } else {
                    error!("Max connection retries reached. Exiting Power.Trade ws client");
                    return ExitCode::FAILURE;
                }
            }
        }
    }
    
    ExitCode::SUCCESS
}

// Tests module including dummy test
#[cfg(test)]
mod tests {
    #[test]
    fn test_dummy_001() {
        assert_eq!(true, true);
    }
}
