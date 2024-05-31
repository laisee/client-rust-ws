extern crate chrono;

use chrono::Utc;
use jwtk::{sign,HeaderAndClaims};
use jwtk::ecdsa::EcdsaPrivateKey;
use log::{error, info};
use serde_json::{Map, Value};
use std::time::Duration;

pub fn generate_access_token(api_key: &str, pkey: String) -> String {

    info!("Loading private key for account {}", api_key);
    let key: EcdsaPrivateKey = match EcdsaPrivateKey::from_pem(pkey.as_bytes()) {
        Ok(my_key) => {
            my_key
        }
        Err(e) => {
            // replace with error handling for invalid/missing private key
            error!("Error while loading private key -> {}", e);
            panic!("Error while loading private key for account {}", api_key);
        }
    };
    let binding: HeaderAndClaims<Map<String, Value>> = HeaderAndClaims::new_dynamic();
    let mut claims: HeaderAndClaims<Map<String, Value>> = binding;

    claims
        .set_iat_now()
        .set_exp_from_now(Duration::from_secs(18000))
        .insert("client", "api".to_owned())
        .insert("sub", api_key.to_owned())
        .insert("nonce",  Utc::now().timestamp()) 
        .set_iss(String::from("app.power.trade"))
        .header_mut().alg ="ES256".to_string().into();

    info!("Adding claims {:?} to signed JWT for account {}", claims, api_key);

    let token: String = match sign( &mut claims, &key) {
        Ok(token) => {
            info!("JWT signed Ok with private key");
            token
        }
        Err(e) => {
            error!("Error signing JWT with private key: {}", e);
            "ERROR-Gen-JWT".to_string()
        }
    };
    token
}