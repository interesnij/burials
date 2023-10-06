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




pub fn user_routes(config: &mut web::ServiceConfig) {
    config.route("/registry/", web::get().to(registry_page));
    config.route("/login/", web::get().to(login_page));
    config.route("/user/{id}/", web::get().to(user_page));


    config.route("/review_user/{id}/", web::get().to(user_page));
    config.route("/comments_user/{id}/", web::get().to(user_page));
    config.route("/organizations_user/{id}/", web::get().to(user_page));
}



