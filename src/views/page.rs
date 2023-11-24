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
};
use sailfish::TemplateOnce;
use diesel::{
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
    PgConnection,
    Connection,
};
use serde::{Deserialize, Serialize};
use crate::schema;
use crate::utils::{
    establish_connection,
    get_request_user,
};


pub fn page_routes(config: &mut web::ServiceConfig) {
    config.route("/", web::get().to(index_page));
    config.route("/info_1/", web::get().to(info_1_page));
}

pub async fn index_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
    let user_id = get_request_user(&req).await;
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        if _request_user.id == 1 {
            User::create_superuser(_request_user.id);
        }
        println!("_request_user {:?}", _request_user.username.clone());
        
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
            struct Template {}
            let body = Template {}
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/main/anon_mainpage.stpl")]
            struct Template {}
            let body = Template {}
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}


pub async fn info_1_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
    let user_id = get_request_user(&req).await;
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
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
            struct Template {}
            let body = Template {}
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/main/anon_info_1.stpl")]
            struct Template {}
            let body = Template {}
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn not_found(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
    let user_id = get_request_user(&req).await;
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/main/404.stpl")]
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
            #[template(path = "mobile/main/404.stpl")]
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
            #[template(path = "desctop/main/anon_404.stpl")]
            struct Template {}
            let body = Template {}
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/main/anon_404.stpl")]
            struct Template {}
            let body = Template {}
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}