use actix_web::{
  HttpRequest,
  web::block,
  HttpResponse,
  http::header::{Header, HeaderValue, TryIntoHeaderValue},
};
use std::{result::Result, env};
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use actix_web_httpauth::headers::authorization::{Authorization, Bearer};
use crate::models::User;

pub fn is_authenticate(token: &String)-> bool {
    return web_local_storage_api::get_item(token).expect("E.").is_some();
}  

pub fn set_token(token: &String, id: &String) {
    let _local_token = web_local_storage_api::set_item(token, id);
    let bearer = Bearer::new(token.as_str());
    let result = bearer.try_into_value();
    assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            HeaderValue::from_static("Bearer".to_string() + bearer)
        );
    //Bearer::new(t);
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

pub async fn get_request_user(req: &HttpRequest) -> Option<User> {

  match Authorization::<Bearer>::parse(req) {
    Ok(ok) => {
      let token = ok.as_ref().token().to_string();
      println!("token {:?}", token.clone());
      return match crate::utils::verify_jwt(token).await {
        Ok(ok) => {
          println!("id {:?}", ok.id);
          Some(crate::utils::get_user(ok.id).expect("E."))
        },
        Err(_) => None,
      }
    },
    Err(_) => None,
  }
}