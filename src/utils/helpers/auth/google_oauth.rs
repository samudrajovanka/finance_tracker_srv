use reqwest::Client as HttpClient;
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation, TokenData};
use super::types::{ GoogleClaims };

pub async fn verify_google_id_token(id_token: &str) -> Result<GoogleClaims, anyhow::Error> {
    let header = decode_header(id_token)?;
    let kid = header.kid.ok_or_else(|| anyhow::anyhow!("Missing kid in header"))?;

    let jwks_url = "https://www.googleapis.com/oauth2/v3/certs";
    let jwks: serde_json::Value = HttpClient::new()
        .get(jwks_url)
        .send()
        .await?
        .json()
        .await?;

    let keys = jwks["keys"].as_array().ok_or_else(|| anyhow::anyhow!("Invalid JWKS format"))?;
    let key = keys.iter().find(|k| k["kid"] == kid).ok_or_else(|| anyhow::anyhow!("Matching kid not found"))?;

    let n = key["n"].as_str().unwrap();
    let e = key["e"].as_str().unwrap();
    let decoding_key = DecodingKey::from_rsa_components(n, e)?;

    let client_id = std::env::var("GOOGLE_CLIENT_ID")?;
    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&[client_id]);
    validation.set_issuer(&["https://accounts.google.com", "accounts.google.com"]);

    let token_data: TokenData<GoogleClaims> = decode::<GoogleClaims>(id_token, &decoding_key, &validation)?;
    Ok(token_data.claims)
}