use chrono::{ DateTime, Duration, Utc };
use data_encoding::BASE64URL_NOPAD;
use dotenvy::dotenv;
use std::env;
use jsonwebtoken::{ encode, decode, Algorithm, DecodingKey, EncodingKey, Header, Validation };
use rand::{ thread_rng, distributions::{ Standard, Distribution } };

use crate::jwt::models::claims::Claims;

// Encrypt the JWT
pub fn encrypt_jwt(secret: &String, claims: Claims) -> Result<String, jsonwebtoken::errors::Error> {
    let header = Header::new(Algorithm::HS256);
    let signature = EncodingKey::from_base64_secret(secret).unwrap();

    let jwt = encode(
        &header,
        &claims,
        &signature,
    )?;

    Ok(jwt)
}

// Decrypt the JWT
pub fn decrypt_jwt(secret: &String, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    // decodes base64, and validates signature and claims
    let claims = decode::<Claims>(
        &token,
        &DecodingKey::from_base64_secret(secret).unwrap(),
        &Validation::new(Algorithm::HS256),
    )?.claims;

    Ok(claims)
}

// generate a unique secret for each user when they're created
pub fn gen_secret() -> String {
    let mut rng = thread_rng();
    let secret: Vec<u8> = Standard.sample_iter(&mut rng).take(256).collect();
    BASE64URL_NOPAD.encode(secret.as_slice())
}

pub fn get_secret() -> String {
    dotenv().ok();
    return env::var("JWT_SECRET").expect("JWT_SECRET must be set");
}

pub fn get_auth_cookie(token: &String) -> String {
    let expires: DateTime<Utc> = Utc::now() + Duration::minutes(1);
    
    format!("token={}; Expires={}; Path=/; SameSite=None; Secure", token, expires.to_rfc2822())
}
