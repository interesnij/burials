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



//-------------------------------------------------------------------------


pub fn deceased_routes(config: &mut web::ServiceConfig) {
    config.route("/all_deceased_place{id}/", web::get().to(all_deceased_place_page));
    config.route("/deceased/{id}/", web::get().to(deceased_page));

    config.route("/edit_deceased/{id}/", web::get().to(edit_deceased_page));
    config.route("/created_deceased/", web::get().to(created_deceased_page));
}

//-------------------------------------------------------------------------




async fn get_request_user_id(req: &HttpRequest) -> Option<i32> { 
    match Authorization::<Bearer>::parse(req) {
        Ok(ok) => {
            let token = ok.as_ref().token().to_string();
            return match verify_jwt(token, "MYSECRETKEY").await {
                Ok(ok) => ok.id,
                Err(_) => None,
            }
        },
        Err(_) => return None,
    }
}

fn get_user(pk: i32) -> User {
    use crate::schema::users::dsl::users;
    let _connection = establish_connection();
    return users
        .filter(schema::users::id.eq(pk))
        .first::<User>(&_connection)
        .expect("E");
}

fn get_content_type<'a>(req: &'a HttpRequest) -> Option<&'a str> {
    return req.headers().get("user-agent")?.to_str().ok();
}
pub fn is_desctop(req: &HttpRequest) -> bool {
    if get_content_type(req).unwrap().contains("Mobile") {
        return false;
    };
    return true;
} 

//-------------------------------------------------------------------------
//Получение всех покойников одного кладбища

pub async fn all_deceased_place_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let is_desctop = is_desctop(&req);
    let _place = block(move || Place::find_by_id(*_id)).await?;
    let _deceased = block(move || Deceased::get_all_deceased()).await?;
    let user_id = get_request_user_id(&req);
    if user_id.is_some() {
        let _request_user = get_user(user_id.unwrap());
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/deceased/all_deceased_place_page.stpl")]
            struct Template {
                request_user:   User,
                place:          Vec<Place>,
                all_deceaseds:  Vec<Deceased>,
            }
            let body = Template {
                request_user:   _request_user,
                place:           _place,
                all_deceaseds:   _deceaseds,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/deceased/all_deceased_place_page.stpl")]
            struct Template {
                request_user:   User,
                place:          Vec<Place>,
                all_deceaseds:  Vec<Deceased>,
            }
            let body = Template {
                request_user:   _request_user,
                place:           _place,
                all_deceaseds:   _deceaseds,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
    else {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/deceased/anon_all_deceased_place_page.stpl")]
            struct Template {
                request_user:   User,
                place:          Vec<Place>,
                all_deceaseds:  Vec<Deceased>,
            }
            let body = Template {
                request_user:   _request_user,
                place:           _place,
                all_deceaseds:   _deceaseds,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/deceased/anon_all_deceased_place_page.stpl")]
            struct Template {
                request_user:   User,
                cats:           Vec<Cat>,
                all_tags:       Vec<SmallTag>,
            }
            let body = Template {
                request_user:   _request_user,
                place:          _place,
                all_deceaseds:  _deceaseds,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

//-------------------------------------------------------------------------
//Получение покойника 


pub async fn deceased_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let is_desctop = is_desctop(&req);
    let _deceased = block(move || Deceased::find_by_id(*_id)).await?;
    let user_id = get_request_user_id(&req);
    if user_id.is_some() {
        let _request_user = get_user(user_id.unwrap());
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/deceased/deceased_page.stpl")]
            struct Template {
                request_user:   User,
                deceased:       Deceased,
            }
            let body = Template {
                request_user:   _request_user,
                deceased:      _deceased,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/deceased/deceased_page.stpl")]
            struct Template {
                request_user:   User,
                deceased:       Deceased,
            }
            let body = Template {
                request_user:   _request_user,
                deceased:       _deceased,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
    else {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/deceased/anon_deceased_page.stpl")]
            struct Template {
                request_user:   User,
                deceased:       Deceased,
            }
            let body = Template {
                request_user:   _request_user,
                deceased:       _deceased,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/deceased/anon_deceased_page.stpl")]
            struct Template {
                request_user:   User,
                deceased:       Deceased,
            }
            let body = Template {
                request_user:   _request_user,
                deceased:       _deceased,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}