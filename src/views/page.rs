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




pub fn page_routes(config: &mut web::ServiceConfig) {
    config.route("/", web::get().to(index_page));
    
    config.route("/info_1/", web::get().to(info_1_page));
    config.route("/info_2/", web::get().to(info_2_page));
    config.route("/info_3/", web::get().to(info_3_page));
    config.route("/info_4/", web::get().to(info_4_page));
    config.route("/info_5/", web::get().to(info_5_page));
    config.route("/info_6/", web::get().to(info_6_page));
    config.route("/info_7/", web::get().to(info_7_page));

}



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




pub async fn index_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let is_desctop = is_desctop(&req);
    let user_id = get_request_user_id(&req);
    if user_id.is_some() {
        let _request_user = get_user(user_id.unwrap());
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/main/mainpage.stpl")]
            struct Template {
                request_user:   User,
            }
            let body = Template {
                request_user:   _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/main/mainpage.stpl")]
            struct Template {
                request_user:   User,
            }
            let body = Template {
                request_user:   _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }

    }

    else {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/main/anon_mainpage.stpl")]
            struct Template {
                request_user:   User,
            }
            let body = Template {
                request_user:   _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/main/anon_mainpage.stpl")]
            struct Template {
                request_user:   User,
            }
            let body = Template {
                request_user:   _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}


pub async fn info_1_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let is_desctop = is_desctop(&req);
    let user_id = get_request_user_id(&req);
    if user_id.is_some() {
        let _request_user = get_user(user_id.unwrap());
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/main/info_1.stpl")]
            struct Template {
                request_user:   User,
            }
            let body = Template {
                request_user:   _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/main/info_1.stpl")]
            struct Template {
                request_user:   User,
            }
            let body = Template {
                request_user:   _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }

    }
    
    else {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/main/anon_info_1.stpl")]
            struct Template {
                request_user:   User,
            }
            let body = Template {
                request_user:   _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/main/anon_info_1.stpl")]
            struct Template {
                request_user:   User,
            }
            let body = Template {
                request_user:   _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}


pub async fn info_2_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let is_desctop = is_desctop(&req);
    let user_id = get_request_user_id(&req);
    if user_id.is_some() {
        let _request_user = get_user(user_id.unwrap());
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/main/info_2.stpl")]
            struct Template {
                request_user:   User,
            }
            let body = Template {
                request_user:   _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/main/info_2.stpl")]
            struct Template {
                request_user:   User,
            }
            let body = Template {
                request_user:   _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }

    }
    
    else {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/main/info_2.stpl")]
            struct Template {
                request_user:   User,
            }
            let body = Template {
                request_user:   _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/main/info_2.stpl")]
            struct Template {
                request_user:   User,
            }
            let body = Template {
                request_user:   _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}


pub async fn info_3_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let is_desctop = is_desctop(&req);
    let user_id = get_request_user_id(&req);
    if user_id.is_some() {
        let _request_user = get_user(user_id.unwrap());
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/main/info_3.stpl")]
            struct Template {
                request_user:   User,
            }
            let body = Template {
                request_user:   _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/main/info_3.stpl")]
            struct Template {
                request_user:   User,
            }
            let body = Template {
                request_user:   _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }

    }
    
    else {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/main/anon_info_3.stpl")]
            struct Template {
                request_user:   User,
            }
            let body = Template {
                request_user:   _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/main/anon_info_3.stpl")]
            struct Template {
                request_user:   User,
            }
            let body = Template {
                request_user:   _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}


pub async fn info_4_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let is_desctop = is_desctop(&req);
    let user_id = get_request_user_id(&req);
    if user_id.is_some() {
        let _request_user = get_user(user_id.unwrap());
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/main/info_4.stpl")]
            struct Template {
                request_user:   User,
            }
            let body = Template {
                request_user:   _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/main/info_4.stpl")]
            struct Template {
                request_user:   User,
            }
            let body = Template {
                request_user:   _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }

    }
    
    else {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/main/anon_info_4.stpl")]
            struct Template {
                request_user:   User,
            }
            let body = Template {
                request_user:   _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/main/anon_info_4.stpl")]
            struct Template {
                request_user:   User,
            }
            let body = Template {
                request_user:   _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}


pub async fn info_5_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let is_desctop = is_desctop(&req);
    let user_id = get_request_user_id(&req);
    if user_id.is_some() {
        let _request_user = get_user(user_id.unwrap());
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/main/info_5.stpl")]
            struct Template {
                request_user:   User,
            }
            let body = Template {
                request_user:   _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/main/info_5.stpl")]
            struct Template {
                request_user:   User,
            }
            let body = Template {
                request_user:   _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }

    }
    
    else {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/main/anon_info_5.stpl")]
            struct Template {
                request_user:   User,
            }
            let body = Template {
                request_user:   _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/main/anon_info_5.stpl")]
            struct Template {
                request_user:   User,
            }
            let body = Template {
                request_user:   _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}


pub async fn info_6_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let is_desctop = is_desctop(&req);
    let user_id = get_request_user_id(&req);
    if user_id.is_some() {
        let _request_user = get_user(user_id.unwrap());
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/main/info_6.stpl")]
            struct Template {
                request_user:   User,
            }
            let body = Template {
                request_user:   _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/main/info_6.stpl")]
            struct Template {
                request_user:   User,
            }
            let body = Template {
                request_user:   _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }

    }
    
    else {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/main/anon_info_6.stpl")]
            struct Template {
                request_user:   User,
            }
            let body = Template {
                request_user:   _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/main/anon_info_6.stpl")]
            struct Template {
                request_user:   User,
            }
            let body = Template {
                request_user:   _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}


pub async fn info_7_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let is_desctop = is_desctop(&req);
    let user_id = get_request_user_id(&req);
    if user_id.is_some() {
        let _request_user = get_user(user_id.unwrap());
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/main/info_7.stpl")]
            struct Template {
                request_user:   User,
            }
            let body = Template {
                request_user:   _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/main/info_7.stpl")]
            struct Template {
                request_user:   User,
            }
            let body = Template {
                request_user:   _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }

    }
    
    else {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/main/anon_info_7.stpl")]
            struct Template {
                request_user:   User,
            }
            let body = Template {
                request_user:   _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/main/anon_info_7.stpl")]
            struct Template {
                request_user:   User,
            }
            let body = Template {
                request_user:   _request_user,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}







