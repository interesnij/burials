use actix_web::web::block;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use std::{result::Result};
//use actix_web_httpauth::extractors::bearer::BearerAuth;


#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub id: i32,
    pub exp: i64,
}

pub async fn gen_jwt (id: i32) -> Result<String, jsonwebtoken::errors::Error> {
    block(move || {
        let header = Header::default();
        let encoding_key = EncodingKey::from_secret("MYSECRETKEY".as_bytes());
        let exp = Utc::now()
            + Duration::days (
                env::var("COOKIE_MAX_AGE")
                .unwrap()
                .parse::<i64>()
                .unwrap()
            );

        let claim = Claims {
            id:  id,
            exp: exp.timestamp(),
        };

        encode(&header, &claim, &encoding_key)
    })
    .await
    .unwrap()
}

pub async fn verify_jwt(_token: String)-> Result<Claims, u16>{
    let claims = block(move || {
        let decoding_key = DecodingKey::from_secret("MYSECRETKEY".as_bytes());

        decode::<Claims>(&_token, &decoding_key, &Validation::default())
    })
    .await
    .unwrap();
    if let Err(_) = claims {
        return Err(403);
    }

    let claims = claims.unwrap().claims;

    if claims.exp < Utc::now().timestamp(){
        return Err(419);
    }

    Ok(claims)
}