use std::env::var;
use log::info;

#[derive(Clone)]
pub struct Config {
    pub server_url: String,
    pub api_key: String,
    pub api_secret: String,
    pub epoch_count: i32,
    pub sleep_duration: u64,
    pub max_retries: i32,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        let server_url = var("PT_SERVER_URL")
            .map_err(|_| "PT_SERVER_URL must be set in the environment or .env file")?;
        
        let api_key = var("PT_API_KEY")
            .map_err(|_| "PT_API_KEY must be set in the environment or .env file")?;
        
        let api_secret = var("PT_API_SECRET")
            .map_err(|_| "PT_API_SECRET must be set in the environment or .env file")?;
        
        let epoch_count = var("PT_EPOCH_COUNT")
            .map_err(|e| format!("Error loading PT_EPOCH_COUNT: {}", e))
            .and_then(|v| v.parse::<i32>()
                .map_err(|e| format!("Error parsing PT_EPOCH_COUNT: {}", e)))?;
        
        let sleep_duration = var("PT_WS_SLEEP")
            .map_err(|e| format!("Error reading PT_WS_SLEEP: {}", e))
            .and_then(|v| v.parse::<u64>()
                .map_err(|e| format!("Error parsing PT_WS_SLEEP: {}", e)))?;
        
        let max_retries = var("PT_MAX_RETRIES")
            .unwrap_or_else(|_| "5".to_string())
            .parse::<i32>()
            .unwrap_or(5);
        
        info!("Configuration loaded: server={}, api_key={}, max_retries={}", 
              server_url, api_key, max_retries);
        
        Ok(Config {
            server_url,
            api_key,
            api_secret,
            epoch_count,
            sleep_duration,
            max_retries,
        })
    }
}
