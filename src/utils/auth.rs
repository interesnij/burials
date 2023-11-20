use actix_web::{
  HttpRequest,
  web::block,
};
use std::{result::Result, env};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use actix_web_httpauth::headers::authorization::{Authorization, Bearer};
use crate::models::User;


pub fn is_authenticate(token: &String)-> bool {
    return web_local_storage_api::get_item(token).expect("E.").is_some();
}  

pub fn set_token(token: &String, id: &String) {
    let _local_token = web_local_storage_api::set_item(token, id);
}

pub async fn remove_token(req: &HttpRequest) -> i16 { 
    match Authorization::<Bearer>::parse(req) {
      Ok(ok) => {
        let token = ok.as_ref().token();
        web_local_storage_api::remove_item(token);
        HttpResponse::Unauthorized().finish();
        return 1;
      },
      Err(_) => 0,
    }
  }

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub id: i32,
    pub exp: i64,
}

pub async fn gen_jwt (
    id: i32,
    secret: &String
) -> Result<String, jsonwebtoken::errors::Error> {
    block(move || {
        let header = Header::default();
        let encoding_key = EncodingKey::from_secret("MY_SECRET".as_bytes());
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
        let decoding_key = DecodingKey::from_secret(b"MY_SECRET");
        decode::<Claims>(&_token, &decoding_key, &Validation::default())
    })
    .await
    .unwrap();
    if let Err(_) = claims {
        return Err(403);
    }

    //log::info!("Headers: {:?}", claims.as_ref().unwrap().header);
    let claims = claims.unwrap().claims;

    if claims.exp < Utc::now().timestamp(){
        return Err(419);
    }

    Ok(claims)
}

pub async fn get_request_user(req: &HttpRequest) -> Option<User> { 
  match Authorization::<Bearer>::parse(req) {
    Ok(ok) => {
      let token = ok.as_ref().token().to_string();
      return match verify_jwt(token).await {
        Ok(ok) => Some(crate::utils::get_user(ok.id).expect("E.")),
        Err(_) => None,
      }
    },
    Err(_) => None,
  }
}