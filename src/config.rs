use std::env::var;
use log::info;

#[derive(Clone, Debug)]
pub struct Config {
    pub server_url: String,
    pub api_key: String,
    pub api_secret: String,
    pub epoch_count: u32,
    pub sleep_duration: u64,
    pub max_retries: u32,
}

impl Config {
    /// Mask sensitive string for logging (show first 4 and last 4 characters)
    fn mask_sensitive(value: &str) -> String {
        if value.len() <= 8 {
            "****".to_string()
        } else {
            format!("{}...{}", &value[..4], &value[value.len()-4..])
        }
    }
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
            .and_then(|v| v.parse::<u32>()
                .map_err(|e| format!("Error parsing PT_EPOCH_COUNT: {}", e)))?;

        let sleep_duration = var("PT_WS_SLEEP")
            .map_err(|e| format!("Error reading PT_WS_SLEEP: {}", e))
            .and_then(|v| v.parse::<u64>()
                .map_err(|e| format!("Error parsing PT_WS_SLEEP: {}", e)))?;

        let max_retries = var("PT_MAX_RETRIES")
            .unwrap_or_else(|_| "5".to_string())
            .parse::<u32>()
            .unwrap_or(5);

        info!("Configuration loaded: server={}, api_key={}, max_retries={}",
              server_url, Self::mask_sensitive(&api_key), max_retries);

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    // Helper to set up test environment
    fn setup_test_env() {
        env::set_var("PT_SERVER_URL", "wss://test.example.com");
        env::set_var("PT_API_KEY", "test_api_key_12345");
        env::set_var("PT_API_SECRET", "test_secret_67890");
        env::set_var("PT_EPOCH_COUNT", "10");
        env::set_var("PT_WS_SLEEP", "5");
        env::set_var("PT_MAX_RETRIES", "3");
    }

    // Helper to clean up test environment
    fn cleanup_test_env() {
        env::remove_var("PT_SERVER_URL");
        env::remove_var("PT_API_KEY");
        env::remove_var("PT_API_SECRET");
        env::remove_var("PT_EPOCH_COUNT");
        env::remove_var("PT_WS_SLEEP");
        env::remove_var("PT_MAX_RETRIES");
    }

    #[test]
    fn test_mask_sensitive_short_string() {
        let result = Config::mask_sensitive("short");
        assert_eq!(result, "****");
    }

    #[test]
    fn test_mask_sensitive_long_string() {
        let result = Config::mask_sensitive("this_is_a_long_api_key_12345");
        assert_eq!(result, "this...2345");
    }

    #[test]
    fn test_mask_sensitive_exactly_8_chars() {
        let result = Config::mask_sensitive("12345678");
        assert_eq!(result, "****");
    }

    #[test]
    fn test_mask_sensitive_9_chars() {
        let result = Config::mask_sensitive("123456789");
        assert_eq!(result, "1234...6789");
    }

    #[test]
    fn test_from_env_success() {
        setup_test_env();

        let config = Config::from_env();
        assert!(config.is_ok());

        let config = config.unwrap();
        assert_eq!(config.server_url, "wss://test.example.com");
        assert_eq!(config.api_key, "test_api_key_12345");
        assert_eq!(config.api_secret, "test_secret_67890");
        assert_eq!(config.epoch_count, 10);
        assert_eq!(config.sleep_duration, 5);
        assert_eq!(config.max_retries, 3);

        cleanup_test_env();
    }

    #[test]
    fn test_from_env_missing_server_url() {
        cleanup_test_env();
        env::set_var("PT_API_KEY", "test_key");
        env::set_var("PT_API_SECRET", "test_secret");
        env::set_var("PT_EPOCH_COUNT", "10");
        env::set_var("PT_WS_SLEEP", "5");

        let result = Config::from_env();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("PT_SERVER_URL"));

        cleanup_test_env();
    }

    #[test]
    fn test_from_env_missing_api_key() {
        cleanup_test_env();
        env::set_var("PT_SERVER_URL", "wss://test.com");
        env::set_var("PT_API_SECRET", "test_secret");
        env::set_var("PT_EPOCH_COUNT", "10");
        env::set_var("PT_WS_SLEEP", "5");

        let result = Config::from_env();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("PT_API_KEY"));

        cleanup_test_env();
    }

    #[test]
    fn test_from_env_missing_api_secret() {
        cleanup_test_env();
        env::set_var("PT_SERVER_URL", "wss://test.com");
        env::set_var("PT_API_KEY", "test_key");
        env::set_var("PT_EPOCH_COUNT", "10");
        env::set_var("PT_WS_SLEEP", "5");

        let result = Config::from_env();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("PT_API_SECRET"));

        cleanup_test_env();
    }

    #[test]
    fn test_from_env_invalid_epoch_count() {
        setup_test_env();
        env::set_var("PT_EPOCH_COUNT", "not_a_number");

        let result = Config::from_env();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("parsing PT_EPOCH_COUNT"));

        cleanup_test_env();
    }

    #[test]
    fn test_from_env_invalid_sleep_duration() {
        setup_test_env();
        env::set_var("PT_WS_SLEEP", "invalid");

        let result = Config::from_env();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("parsing PT_WS_SLEEP"));

        cleanup_test_env();
    }

    #[test]
    fn test_from_env_default_max_retries() {
        cleanup_test_env();
        env::set_var("PT_SERVER_URL", "wss://test.com");
        env::set_var("PT_API_KEY", "test_key");
        env::set_var("PT_API_SECRET", "test_secret");
        env::set_var("PT_EPOCH_COUNT", "10");
        env::set_var("PT_WS_SLEEP", "5");
        // Don't set PT_MAX_RETRIES

        let config = Config::from_env().unwrap();
        assert_eq!(config.max_retries, 5); // Should default to 5

        cleanup_test_env();
    }

    #[test]
    fn test_from_env_invalid_max_retries_uses_default() {
        setup_test_env();
        env::set_var("PT_MAX_RETRIES", "not_a_number");

        let config = Config::from_env().unwrap();
        assert_eq!(config.max_retries, 5); // Should default to 5 on parse error

        cleanup_test_env();
    }
}
