use actix_web::{
    web,
    web::block,
    HttpRequest,
    HttpResponse,
    error::InternalError,
    http::StatusCode,
};
use crate::errors::Error;
use actix_web_httpauth::headers::authorization::{Authorization, Bearer};
use crate::models::{
    Deceased,
    Geo,
    Organization,
    Places,
    Reiew,
    Service,
    Words,
    User,
};
use sailfish::TemplateOnce;
use diesel::{
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
    PgConnection,
    Connection,
};
use actix_multipart::{Field, Multipart};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::{
    io::Write,
    fs::create_dir_all,
    str,
};
use crate::schema;
use crate::utils::establish_connection;


pub fn deceased_routes(config: &mut web::ServiceConfig) {
    config.route("/deceased/{id}/", web::get().to(deceased_page));
    config.route("/all_deceased_place{id}/", web::get().to(all_deceased_place_page));
}




