use hmac::{Hmac, Mac};
use jwt::{AlgorithmType, Error, Header, SignWithKey, Token, VerifyWithKey};
use std::collections::BTreeMap;
use chrono::prelude::*;
use sha2::Sha256;

pub fn create_access_token(username: &str) ->  Result<String, Error> { 
    let secret = std::env::var("SECRET").expect("SECRET must be set.");
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes())?;
    let header = Header {
        algorithm: AlgorithmType::Hs256,
        ..Default::default()
    };
    let current_local: DateTime<Local> = Local::now();
    let custom_format = current_local.format("%Y-%m-%d").to_string();
    let mut claims = BTreeMap::new();
    claims.insert("name", username);
    claims.insert("sub", "login");
    claims.insert("iat", &custom_format);
    let token = Token::new(header, claims).sign_with_key(&key)?.as_str().to_string();
    let var_name = Ok(token);
    var_name
}

pub fn verify_jwt_token(username: &str, access_token: &str) -> Result<bool, Error> {
    let secret = std::env::var("SECRET").expect("SECRET must be set.");
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes())?;
    let token: Token<Header, BTreeMap<String, String>, _> = access_token.verify_with_key(&key)?;
    let header = token.header();
    let claims = token.claims();
    assert_eq!(header.algorithm, AlgorithmType::Hs256);
    assert_eq!(claims["sub"], "login");
    assert_eq!(claims["name"], username);
    Ok(true)
}