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
    let t = token.to_string();
    let bearer = Bearer::new(t.clone());
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
      println!("cookie None");
        return None;
    }
    let _cookie = _cookie_res.expect("E.").to_str().ok();
    if _cookie.is_some() {
        for c in _cookie.unwrap().split("; ").collect::<Vec<&str>>().iter() {
          println!("c {:?}", c.clone());
            let split_c: Vec<&str> = c.split("=").collect();
            println!("c[0] {:?}", split_c[0]);
            if split_c[0] == "userrr" {
                let user_id = split_c[1].parse().unwrap();
                let __user = crate::utils::get_user(user_id);
                if __user.is_ok() {
                    let _user = __user.expect("E.");
                    println!("user_id {:?}", &_user.id);
                    return Some(_user);
                }
            }
        }
    }
    return None;
}