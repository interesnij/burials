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


pub fn user_routes(config: &mut web::ServiceConfig) {

    config.route("/user/{id}/", web::get().to(user_page));
    config.route("/setting_user/{id}/", web::get().to(setting_user_page));


    config.route("/delete_user/{id}/", web::post().to(delete_user_page));
    config.route("/edit_user/{id}/", web::post().to(edit_user_page));
    config.route("/create_user/", web::post().to(create_user_page));
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
//Получение пользователя


pub async fn user_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let is_desctop = is_desctop(&req);
    let _user = block(move || User::find_by_id(*_id)).await?;
    let user_id = get_request_user_id(&req);
    if user_id.is_some() {
        let _request_user = get_user(user_id.unwrap());
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

//-------------------------------------------------------------------------
//Настройки пользователя


pub async fn setting_user_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let is_desctop = is_desctop(&req);
    let _user = block(move || User::find_by_id(*_id)).await?;
    let user_id = get_request_user_id(&req);
    if user_id.is_some() {
        let _request_user = get_user(user_id.unwrap());
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/user/setting_user_page.stpl")]
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
            #[template(path = "mobile/user/setting_user_page.stpl")]
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
            #[template(path = "desctop/user/anon_setting_user_page.stpl")]
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



pub async fn create_user_page(req: HttpRequest, mut payload: Multipart) -> impl Responder {
    let user_id = get_request_user_id(&req);
    if user_id.is_some() {
        let _request_user = get_user(user_id.unwrap());
        if _request_user.is_admin() {
            let form = user_form(payload.borrow_mut(), _request_user.id).await;
            User::create_user(form);
        }
    };
    HttpResponse::Ok()
}

pub async fn edit_user_page(req: HttpRequest, _id: web::Path<i32>) -> impl Responder {
    let user_id = get_request_user_id(&req);
    if user_id.is_some() {
        let _request_user = get_user(user_id.unwrap());
        let _user = block(move || User::find_by_id(*_id)).await?; 
        if _request_user.id == user.user_id {
            let form = user_form(payload.borrow_mut(), _request_user.id).await;
            User.edit_user(form);
        }
    };
    HttpResponse::Ok()
}
pub async fn delete_user_page(req: HttpRequest, _id: web::Path<i32>) -> impl Responder {
    let user_id = get_request_user_id(&req);
    if user_id.is_some() {
        let _request_user = get_user(user_id.unwrap());
        let _user = block(move || User::find_by_id(*_id)).await?; 
        if _request_user.id == user.user_id {
            let form = user_form(payload.borrow_mut(), _request_user.id).await;
            User.delete_user();
        }
    };
    HttpResponse::Ok()
}

//---------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct UploadedFiles {
    pub name: String,
    pub path: String,
}
impl UploadedFiles {
    fn new(filename: String, owner_id: i32) -> UploadedFiles {
        use chrono::Datelike;

        let now = chrono::Local::now().naive_utc();
        let format_folder = format!(
            "./media/{}/{}/{}/{}/",
            owner_id.to_string(),
            now.year().to_string(),
            now.month().to_string(),
            now.day().to_string(),
        );
        let format_path = format_folder.clone() + &filename.to_string();
        // вариант для https
        let create_path = format_folder.replace("./", "/my/");
        // вариант для debug
        //let create_path = format_folder.replace("./", "/");
        create_dir_all(create_path).unwrap();

        UploadedFiles {
            name: filename.to_string(),
            path: format_path.to_string(),
        }
    }
}
