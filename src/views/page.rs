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




pub fn pages_routes(config: &mut web::ServiceConfig) {
    config.route("/", web::get().to(index_page));
    config.route("/about/", web::get().to(about_page));
    config.route("/faq/", web::get().to(faq_page));

    config.route("/add_deceased_place/{id}/", web::get().to(add_deceased_page));
    config.route("/add_place_city/{id}/", web::get().to(add_place_page));
    config.route("/add_organization/", web::get().to(add_organization_page));
    config.route("/add_service_organization/{id}/", web::get().to(add_service_page));
}












