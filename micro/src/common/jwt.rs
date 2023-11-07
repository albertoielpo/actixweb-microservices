use common_lib::utils::date::unix_timestamp;
use hmac::{Hmac, Mac};
use jwt::{Error, SignWithKey, VerifyWithKey};
use lazy_static::lazy_static;
use log::error;
use sha2::Sha256;
use std::{collections::BTreeMap, env};

lazy_static! {
    static ref JWT_SECRET: String =
        env::var("JWT_SECRET").unwrap_or("2c0a7176-182b-4834-bf42-836c2c7c5d35".to_owned());
    static ref JWT_SUB: String = env::var("JWT_SUB").unwrap_or("admin@changeit.please".to_owned());
    static ref JWT_ISS: String = env::var("JWT_ISS").unwrap_or("changeit.please".to_owned());
    static ref JWT_AUD: String =
        env::var("JWT_AUD").unwrap_or("https://changeit.please".to_owned());
}

pub fn sign(username: &str) -> Result<String, Error> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(JWT_SECRET.as_bytes())?;
    let mut claims: BTreeMap<&str, &str> = BTreeMap::new();

    let now = unix_timestamp();
    let now_str = now.to_string();

    let exp = now + 3600000; //1 hour
    let exp_str = exp.to_string();

    claims.insert("us", &username);
    claims.insert("role", "public"); // define role if needed
    claims.insert("iat", &now_str); //issue timestamp
    claims.insert("exp", &exp_str);
    claims.insert("aud", &JWT_AUD);
    claims.insert("iss", &JWT_ISS);
    claims.insert("sub", &JWT_SUB);

    let token = claims.sign_with_key(&key)?;
    Ok(token)
}

pub fn verify(token: &str) -> Result<(), Error> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(JWT_SECRET.as_bytes())?;
    let claims: BTreeMap<String, String> = token.verify_with_key(&key)?; //get data
    let exp_token_date = claims["exp"].parse::<u128>().unwrap_or(0);
    let now = unix_timestamp();
    if now > exp_token_date {
        error!("Token expired!");
        //a generic error could be raise here.. the interceptor will catch and put +401
        return Err(Error::InvalidSignature);
    }

    Ok(())
}
