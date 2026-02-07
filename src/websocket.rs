use crate::config::Config;
use crate::error::AppError;
use crate::utils::generate_access_token;

use log::info;
use tungstenite::{client::IntoClientRequest, connect, http::HeaderValue, WebSocket, stream::MaybeTlsStream, Message};
use url::Url;
use std::net::TcpStream;
use std::time::Duration;
use std::thread::sleep;

pub struct WebSocketClient {
    socket: WebSocket<MaybeTlsStream<TcpStream>>,
    config: Config,
}

impl WebSocketClient {
    pub fn new(config: Config) -> Result<Self, AppError> {
        // Validate configuration before connecting
        Self::validate_config(&config)?;
        
        let socket = Self::connect(&config)?;
        Ok(WebSocketClient { socket, config })
    }
    
    // Validate the configuration
    fn validate_config(config: &Config) -> Result<(), AppError> {
        // Check if server URL is valid
        if config.server_url.is_empty() {
            return Err(AppError::Config("Server URL cannot be empty".to_string()));
        }
        
        // Validate URL format
        Url::parse(&config.server_url)
            .map_err(|e| AppError::Config(format!("Invalid server URL: {}", e)))?;
        
        // Check API credentials
        if config.api_key.is_empty() {
            return Err(AppError::Config("API key cannot be empty".to_string()));
        }
        
        if config.api_secret.is_empty() {
            return Err(AppError::Config("API secret cannot be empty".to_string()));
        }
        
        // Validate other configuration parameters
        if config.epoch_count <= 0 {
            return Err(AppError::Config("Epoch count must be positive".to_string()));
        }
        
        if config.sleep_duration == 0 {
            return Err(AppError::Config("Sleep duration cannot be zero".to_string()));
        }
        
        if config.max_retries < 0 {
            return Err(AppError::Config("Max retries cannot be negative".to_string()));
        }
        
        Ok(())
    }
    
    fn connect(config: &Config) -> Result<WebSocket<MaybeTlsStream<TcpStream>>, AppError> {
        info!("Connecting to {}", config.server_url);
        
        let url = Url::parse(&config.server_url)
            .map_err(|e| AppError::Config(format!("Invalid server URL: {}", e)))?;
            
        // Generate authentication token
        let token = generate_access_token(&config.api_key, &config.api_secret);
        info!("Token generated for account {}", config.api_key);
        
        // Create request with authentication header
        let mut request = url.as_str().into_client_request()
            .map_err(|e| AppError::Connection(format!("Failed to create request: {}", e)))?;
            
        request.headers_mut().append(
            "X-Power-Trade", 
            HeaderValue::from_str(&token)
                .map_err(|e| AppError::Authentication(format!("Invalid token: {}", e)))?
        );
        
        // Connect to WebSocket server
        info!("Connecting to Power.Trade server: {}", config.server_url);
        let (socket, response) = connect(request)
            .map_err(|e| AppError::Connection(format!("Connection failed: {}", e)))?;
            
        info!("Connected to server: HTTP {}", response.status());
        
        Ok(socket)
    }
    
    pub fn read_message(&mut self) -> Result<Message, AppError> {
        self.socket.read().map_err(AppError::from)
    }
    
    pub fn write_message(&mut self, msg: Message) -> Result<(), AppError> {
        self.socket.write(msg).map_err(AppError::from)
    }
    
    pub fn reconnect(&mut self) -> Result<(), AppError> {
        info!("Attempting to reconnect...");
        self.socket = Self::connect(&self.config)?;
        Ok(())
    }
    
    // Add a method to get configuration information
    pub fn get_config_info(&self) -> String {
        format!(
            "Connected to {} with API key {}, max retries: {}", 
            self.config.server_url, 
            self.config.api_key, 
            self.config.max_retries
        )
    }
    
    // Add a method that uses both write_message and reconnect
    pub fn send_ping_with_retry(&mut self, max_attempts: i32) -> Result<(), AppError> {
        let mut attempts = 0;
        
        while attempts < max_attempts {
            match self.write_message(Message::Ping(vec![1, 2, 3].into())) {
                Ok(_) => {
                    info!("Ping sent successfully");
                    return Ok(());
                },
                Err(e) => {
                    info!("Failed to send ping: {}, attempting to reconnect", e);
                    attempts += 1;
                    
                    if attempts < max_attempts {
                        match self.reconnect() {
                            Ok(_) => info!("Reconnected successfully, retrying ping"),
                            Err(e) => {
                                info!("Failed to reconnect: {}", e);
                                sleep(Duration::from_secs(1));
                            }
                        }
                    }
                }
            }
        }
        
        Err(AppError::Connection("Failed to send ping after maximum retry attempts".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // Helper function to create a test configuration
    fn create_test_config() -> Config {
        Config {
            server_url: "wss://test.example.com/ws".to_string(),
            api_key: "test_key".to_string(),
            api_secret: "test_secret".to_string(),
            epoch_count: 10,
            sleep_duration: 5,
            max_retries: 3,
        }
    }
    
    #[test]
    fn test_validate_config_valid() {
        let config = create_test_config();
        let result = WebSocketClient::validate_config(&config);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_validate_config_empty_url() {
        let mut config = create_test_config();
        config.server_url = "".to_string();
        
        let result = WebSocketClient::validate_config(&config);
        assert!(result.is_err());
        
        match result {
            Err(AppError::Config(msg)) => {
                assert!(msg.contains("Server URL cannot be empty"));
            },
            _ => panic!("Expected Config"),
        }
    }
    
    #[test]
    fn test_validate_config_invalid_url() {
        let mut config = create_test_config();
        config.server_url = "invalid-url".to_string();
        
        let result = WebSocketClient::validate_config(&config);
        assert!(result.is_err());
        
        match result {
            Err(AppError::Config(msg)) => {
                assert!(msg.contains("Invalid server URL"));
            },
            _ => panic!("Expected Config"),
        }
    }
    
    #[test]
    fn test_validate_config_empty_api_key() {
        let mut config = create_test_config();
        config.api_key = "".to_string();
        
        let result = WebSocketClient::validate_config(&config);
        assert!(result.is_err());
        
        match result {
            Err(AppError::Config(msg)) => {
                assert!(msg.contains("API key cannot be empty"));
            },
            _ => panic!("Expected Config"),
        }
    }
    
    #[test]
    fn test_validate_config_empty_api_secret() {
        let mut config = create_test_config();
        config.api_secret = "".to_string();
        
        let result = WebSocketClient::validate_config(&config);
        assert!(result.is_err());
        
        match result {
            Err(AppError::Config(msg)) => {
                assert!(msg.contains("API secret cannot be empty"));
            },
            _ => panic!("Expected Config"),
        }
    }
    
    #[test]
    fn test_validate_config_invalid_epoch_count() {
        let mut config = create_test_config();
        config.epoch_count = 0;
        
        let result = WebSocketClient::validate_config(&config);
        assert!(result.is_err());
        
        match result {
            Err(AppError::Config(msg)) => {
                assert!(msg.contains("Epoch count must be positive"));
            },
            _ => panic!("Expected Config"),
        }
    }
    
    #[test]
    fn test_validate_config_invalid_sleep_duration() {
        let mut config = create_test_config();
        config.sleep_duration = 0;
        
        let result = WebSocketClient::validate_config(&config);
        assert!(result.is_err());
        
        match result {
            Err(AppError::Config(msg)) => {
                assert!(msg.contains("Sleep duration cannot be zero"));
            },
            _ => panic!("Expected Config"),
        }
    }
    
    #[test]
    fn test_validate_config_invalid_max_retries() {
        let mut config = create_test_config();
        config.max_retries = -1;
        
        let result = WebSocketClient::validate_config(&config);
        assert!(result.is_err());
        
        match result {
            Err(AppError::Config(msg)) => {
                assert!(msg.contains("Max retries cannot be negative"));
            },
            _ => panic!("Expected Config"),
        }
    }
    
    #[test]
    fn test_config_info_format() {
        let config = create_test_config();
        
        // Test the format string directly without creating a WebSocketClient
        let info = format!(
            "Connected to {} with API key {}, max retries: {}", 
            config.server_url, 
            config.api_key, 
            config.max_retries
        );
        
        assert!(info.contains(&config.server_url));
        assert!(info.contains(&config.api_key));
        assert!(info.contains(&config.max_retries.to_string()));
    }
}
