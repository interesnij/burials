use actix_web::{
    HttpRequest,
    HttpResponse,
    Responder,
    web,
    web::{block, Data, Json},
};
use crate::schema;
use serde::{Deserialize, Serialize};

use crate::utils::{
    establish_connection,
    get_request_user,
};
use crate::diesel::{
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
};
use actix_multipart::Multipart;
use std::str;
use std::borrow::BorrowMut;
use crate::errors::Error;


pub fn progs_routes(config: &mut web::ServiceConfig) {
    config.route("/feedback/", web::post().to(create_feedback));
}

pub async fn create_feedback(mut payload: actix_multipart::Multipart) -> impl Responder {
    use crate::schema::feedbacks;
    use crate::models::NewFeedback;
    use crate::utils::feedback_form;

    let _connection = establish_connection();
    let form = feedback_form(payload.borrow_mut()).await;
    let new_feedback = NewFeedback {
        username: form.username.clone(),
        email:    form.email.clone(),
        message:  form.message.clone()
    };
    diesel::insert_into(feedbacks::table)
        .values(&new_feedback)
        .execute(&_connection)
        .expect("E.");
    return HttpResponse::Ok();
}