use actix_web::web;

use crate::views::{
    auth,
    deceaseds,
    admin,
    //organization,
    page,
    place,
    //progs,
    //service,
    //user,
};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
    .configure(auth::auth_routes)
    .configure(deceaseds::deceased_routes)
    //.configure(organization::organization_routes)
    .configure(page::page_routes)
    .configure(admin::admin_routes)
    .configure(place::place_routes)
    //.configure(progs::progs_routes)
    //.configure(service::service_routes)
    //.configure(user::user_routes)
    ;
}
