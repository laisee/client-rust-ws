#![warn(unreachable_patterns)]
include!(concat!(env!("OUT_DIR"), "/build_date.rs"));

mod utils;

use std::process::ExitCode;
use std::{thread::sleep, time::Duration};
use std::fs::File;
use std::env::var;

use clap::{value_parser, Arg, ArgAction, Command, ValueEnum};
use utils::generate_access_token;
use log::{error, info};
use simplelog::*;
use tungstenite::{client::IntoClientRequest, connect, http::HeaderValue, Message};
use url::Url;

/// run()
///
/// # Panics
///
/// Panics if env settings not found or not valid
///
/// Parameters: None
/// 
/// Return value: tungstenite::Result 
/// 
fn run() -> tungstenite::Result<()> {
    
    // load target server based on env setting
    let pt_server = var("PT_SERVER")
        .expect("PT_SERVER must be set in the environment or .env file");
    info!("connecting to {:?}", pt_server);

    // verify the server address is valid
    let url: Url = match Url::parse(&pt_server) {
        Ok(url) => url,
        Err(error) => {
            panic!("Error parsing server address {:?} -> {:?}", &pt_server, error);
        }
    };

    // load Power.Trade API key && private key from env
    let api_key= var("PT_API_KEY")
        .expect("PT_API_KEY must be set in the environment or .env file");
    info!("PT_API_KEY: {:?}", api_key);
    
    let api_secret= var("PT_API_SECRET")
        .expect("PT_API_SECRET must be set in the environment or .env file");

    // generate JWT token for authenticating at server
    let token: String = generate_access_token(&api_key, api_secret);

    // log first X chars to assist with debugging issues
    info!("Token generated for account {:?}\n{:?} ", api_key, token.clone().truncate(50));

    // setup WS request with required Power.Trade header 
    let mut request = url.into_client_request()?;
    request.headers_mut().append("X-Power-Trade", HeaderValue::from_str(&token).unwrap());

    info!("Connecting to Power.Trade server: {}", &pt_server);
    println!("Connecting to Power.Trade server: {}", &pt_server);

    let (mut socket, response) = connect(request)?;
    println!("Response from server {:?} -> {:?}", pt_server, response.status());

    info!("Power.Trade websocket client now active for server {}", &pt_server);
    println!("Power.Trade websocket client now active for server {}", &pt_server);

    // setup loop for checking received messages 
    // n.b. to be replaced by event-driven code
    let mut count: i32 = 0;
    const DEFAULT_EPOCH: i32 = 10000000;
    let max_epoch: i32 = match var("PT_EPOCH_COUNT") {
        Ok(max_epoch_str) => {
            match max_epoch_str.parse::<i32>() {
                Ok(value) => value,
                Err(error) => {
                    println!("Error['{}'] while parsing 'PT_EPOCH_COUNT' - assigning default value of 10000000", error);
                    DEFAULT_EPOCH
                }
            }
        },
        Err(error) => {
            println!("Error['{}'] while loading 'PT_EPOCH_COUNT' from environment - assigning default value of 10000000", error);
            DEFAULT_EPOCH // default value for epoch count
        }
    };
    
    //
    // loop on message receive -> TODO replace by event-driven style
    //
    loop {
        let msg: Message = socket.read()?;
        info!("Received msg: {}", msg);
        info!("Power.Trade websocket client sleeping [{} of {} epochs]", count, max_epoch);
        sleep( Duration::from_secs(2));
        count += 1;
        if count >= max_epoch {
            println!("Power.Trade websocket client closing after count of {} epochs exceeded", count);
            info!("\nPower.Trade websocket client closing after count of {} epochs exceeded\n", count);
            break;
        }
    }
    Ok(())
}

fn main()  -> ExitCode {

    let version_info: String = format!("version {} built on {}", env!("CARGO_PKG_VERSION"), BUILD_DATE);
    let version_info: &'static str = Box::leak(version_info.into_boxed_str());

    println!("Starting websocket client for power.trade [{:?}]", version_info);

    // Initialize the loggeing set file - replace hardcoded name with value from env settings (.env file)
    CombinedLogger::init(vec![WriteLogger::new(LevelFilter::Info, Config::default(), File::create("app.log").unwrap())]).unwrap();

    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
    enum Environment {
        Development,
        Test,
        Production
    }
    // check ENV to be run && set env config file name based on ENV settings
    let matches: clap::ArgMatches = Command::new("Power.Trade Websocket Client")
        .version(version_info)
        .about("Handles the env argument")
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
        .arg(Arg::new("custom-help")
            .short('?')
            .action(ArgAction::Help)
            .display_order(100)  // Don't sort
            .help("Alt help")
        )
        .get_matches();

    // Retrieve the value of env
    let pt_env: &Environment = matches.get_one::<Environment>("env").expect("env is required");

    // Handle different values of env
    match pt_env {
        Environment::Development => {
            println!("Environment is set to DEV");
            // Load environment variables from .env file
            dotenv::from_filename(".env.dev").expect("Failed to load env values from file '.env.dev'");
        },
        Environment::Test => {
            println!("Environment is set to TEST");
            dotenv::from_filename(".env.test").expect("Failed to load env values from file '.env.test'");
        },
        Environment::Production => {
            println!("Environment is set to PROD");
            dotenv::from_filename(".env.prod").expect("Failed to load env values from file '.env.prod'");
        },
    }

    //
    // Use loop to retry connections if network issues etc
    // - will panic if more than 5 attempts fail
    let mut retries: i32 = 5; // replace with env var
    loop {
        match run() {
            Ok(_) => break,
            Err(e) => {
                error!("Error: {:?}", e);
                if retries > 0 {
                    retries -= 1;
                    info!("Retrying... attempts left: {}", retries);
                    std::thread::sleep(Duration::from_secs(5));
                } else {
                    panic!("Max connection retries reached. Exiting Power.Trade ws client");
                }
            }
        }
    }
    ExitCode::from(0) // set exit status for any monitoring app on this code
}

// Tests module including dummy test
#[cfg(test)]
mod tests {

    #[test]
    fn test_dummy_001() {
        assert_eq!(true, true);
    }
}
