use actix_web::{
    web,
    web::block,
    HttpRequest,
    HttpResponse,
    error::InternalError,
    http::StatusCode,
};
use crate::errors::Error;
use crate::models::{
    User,
    Geo,
    Organization,
    Places,
    Reiew,
    Service,
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
use crate::schema;
use crate::utils::establish_connection;



//-------------------------------------------------------------------------


pub fn user_routes(config: &mut web::ServiceConfig) {
    config.route("/user/{id}/", web::get().to(user_page));
}



pub async fn user_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let is_desctop = is_desctop(&req);
    let _user = block(move || User::find_by_id(*_id)).await?;
    let user_id = get_request_user(&req);
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/user/user_page.stpl")]
            struct Template {
                request_user:   User,
                user:       User,
            }
            let body = Template {
                request_user:   _request_user,
                user:      _user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/user/user_page.stpl")]
            struct Template {
                request_user:   User,
                user:       User,
            }
            let body = Template {
                request_user:   _request_user,
                user:       _user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
    else {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/user/anon_user_page.stpl")]
            struct Template {
                request_user:   User,
                user:       User,
            }
            let body = Template {
                request_user:   _request_user,
                user:       _user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/user/anon_user_page.stpl")]
            struct Template {
                request_user:   User,
                user:       User,
            }
            let body = Template {
                request_user:   _request_user,
                user:       _user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}