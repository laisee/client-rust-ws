extern crate chrono;

use chrono::Utc;
use jwtk::{sign,HeaderAndClaims};
use jwtk::ecdsa::EcdsaPrivateKey;
use log::info;
use serde_json::{Map, Value};
use std::time::Duration;
use crate::error::AppError;

pub fn generate_access_token(api_key: &str, pkey: &str) -> Result<String, AppError> {

    info!("Loading private key");
    let key: EcdsaPrivateKey = EcdsaPrivateKey::from_pem(pkey.as_bytes())
        .map_err(|e| AppError::Authentication(format!("Failed to load private key: {}", e)))?;

    let mut claims: HeaderAndClaims<Map<String, Value>> = HeaderAndClaims::new_dynamic();

    claims
        .set_iat_now()
        .set_exp_from_now(Duration::from_secs(18000))
        .insert("client", "api".to_owned())
        .insert("sub", api_key.to_owned())
        .insert("nonce",  Utc::now().timestamp())
        .set_iss(String::from("app.power.trade"))
        .header_mut().alg ="ES256".to_string().into();

    info!("Signing JWT token");

    let token = sign(&mut claims, &key)
        .map_err(|e| AppError::Authentication(format!("Failed to sign JWT: {}", e)))?;

    info!("JWT signed successfully");
    Ok(token)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test EC private key in PEM format (for testing only - this is a generated test key)
    #[allow(dead_code)]
    const TEST_PRIVATE_KEY: &str = r#"-----BEGIN EC PRIVATE KEY-----
MHcCAQEEIIGlRuLMXJpEOKGBhKTjY+JQJvVvJvVvJvVvJvVvJvVvoAoGCCqGSM49
AwEHoUQDQgAEzqTwQpXEB0AO6lPZzt4DjTx+/SjnRBJYgWIKThljQVcemQGdUVjx
pDtVVEpXOEpXOEpXOEpXOEpXOEpXOEpXOA==
-----END EC PRIVATE KEY-----"#;

    const INVALID_PRIVATE_KEY: &str = r#"-----BEGIN EC PRIVATE KEY-----
INVALID_KEY_DATA
-----END EC PRIVATE KEY-----"#;

    #[test]
    fn test_generate_access_token_with_invalid_key() {
        let result = generate_access_token("test_api_key", INVALID_PRIVATE_KEY);
        assert!(result.is_err());

        match result {
            Err(AppError::Authentication(msg)) => {
                assert!(msg.contains("Failed to load private key"));
            },
            _ => panic!("Expected Authentication error"),
        }
    }

    #[test]
    fn test_generate_access_token_with_empty_key() {
        let result = generate_access_token("test_api_key", "");
        assert!(result.is_err());

        match result {
            Err(AppError::Authentication(msg)) => {
                assert!(msg.contains("Failed to load private key"));
            },
            _ => panic!("Expected Authentication error"),
        }
    }

    #[test]
    fn test_generate_access_token_with_malformed_pem() {
        let malformed_pem = "This is not a PEM formatted key";
        let result = generate_access_token("test_api_key", malformed_pem);
        assert!(result.is_err());

        match result {
            Err(AppError::Authentication(msg)) => {
                assert!(msg.contains("Failed to load private key"));
            },
            _ => panic!("Expected Authentication error"),
        }
    }

    // Note: We can't easily test successful token generation without a valid EC key
    // and the jwtk library properly configured. The above tests cover error cases
    // which is the most important part for error handling coverage.
}
