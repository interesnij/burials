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



pub fn organizatios_routes(config: &mut web::ServiceConfig) {
    config.route("/all_organizatios_place/{id}/", web::get().to(all_organizations_place_page));
    config.route("/all_organizatios_city/{id}/", web::get().to(all_organizations_city_page));
    config.route("/all_organizatios_region/{id}/", web::get().to(all_organizations_region_page));
    config.route("/all_organizatios_country/{id}/", web::get().to(all_organizations_country_page));


    config.route("/all_service_organization/{id}/", web::get().to(all_service_organizations_page));
    config.route("/all_service_place/{id}/", web::get().to(all_service_place_page));
}



