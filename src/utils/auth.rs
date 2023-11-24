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
    //let _local_token = web_local_storage_api::set_item(token, id);
    //let t = token.to_string();
    //let bearer = Bearer::new(t.clone());
    //let ne = &("Bearer".to_string() + &token.to_string());
    //let result = bearer.try_into_value();
    //assert!(result.is_ok());
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
  let _cookie_res = req.headers().get("cookie");

    if _cookie_res.is_none() {
        return None;
    }
    let _cookie = _cookie_res.expect("E.").to_str().ok();
    if _cookie.is_some() {
        for c in _cookie.unwrap().split("; ").collect::<Vec<&str>>().iter() {
            let split_c: Vec<&str> = c.split("=").collect();
            if split_c[0] == "user" {
                let user_id = split_c[1].parse().unwrap();
                let _user = crate::utils::get_user(user_id);
                if _user.is_ok() {
                  return Some(_user.expect("E."));
                }
            }
        }
    }
    return None;
  //match Authorization::<Bearer>::parse(req) {
  //  Ok(ok) => {
  //    let token = ok.as_ref().token().to_string();
  //    println!("token {:?}", token.clone());
  //    return match crate::utils::verify_jwt(token).await {
  //      Ok(ok) => {
  //        println!("id {:?}", ok.id);
  //        Some(crate::utils::get_user(ok.id).expect("E."))
  //      },
  //      Err(_) => None,
  //    }
  //  },
  //  Err(_) => None,
  //}
}