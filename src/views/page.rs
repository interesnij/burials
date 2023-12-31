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
    User, Deceased,
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
    config.route("/main_search", web::get().to(main_search));
    config.route("/image/{id}/", web::get().to(image_page));
}

pub async fn index_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
    let user_id = get_request_user(&req).await;
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        //if _request_user.id == 1 || _request_user.id == 2 {
        //User::create_superuser(_request_user.id);
        //}
        //println!("_request_user {:?}", _request_user.username.clone());
        
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

    }

    else {
        println!("anon");
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
            #[template(path = "desctop/main/anon_mainpage.stpl")]
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
            #[template(path = "desctop/main/anon_404.stpl")]
            struct Template {}
            let body = Template {}
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}


#[derive(Deserialize)]
pub struct SeacrhData {
    pub first_name:  Option<String>,
    pub middle_name: Option<String>,
    pub last_name:   Option<String>,
    pub birth_date:  Option<chrono::NaiveDate>,
    pub death_date:  Option<chrono::NaiveDate>,
    pub location:    Option<String>,
}  
pub async fn main_search(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let params_some = web::Query::<SeacrhData>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        if params.last_name.is_none() || params.first_name.is_none() {
            return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("no last_name"));
        }
        let birth_date: Option<chrono::NaiveDate>;
        let death_date: Option<chrono::NaiveDate>;
        let mut location: Option<String>; 
        let birth_date_dd = params.birth_date.is_some() && params.birth_date.unwrap().format("%Y-%m-%d").to_string() == "2023-11-25".to_string();
        let death_date_dd = params.death_date.is_some() && params.death_date.unwrap().format("%Y-%m-%d").to_string() == "2023-11-25".to_string();
        //println!("birth_date_dd {:?}", birth_date_dd);
        //println!("death_date_dd {:?}", death_date_dd);
        //println!("birth_format {:?}", params.birth_date.unwrap().format("%Y-%m-%d").to_string());
        //println!("death_format {:?}", params.death_date.unwrap().format("%Y-%m-%d").to_string());
        if birth_date_dd {
            birth_date = None;
        }
        else {
            birth_date = params.birth_date;
        }
        if death_date_dd {
            death_date = None;
        }
        else {
            death_date = params.death_date;
        }

        if params.location.is_none() {
            println!("location None!!!");
            location = None;
        }
        else {
            location = params.location.clone();
            if &location.as_deref().unwrap().to_string() == &"xxx".to_string() {
                println!("location empty!!!");
                location = None;
            }
        }

        let user_id = get_request_user(&req).await;
        let object_list = Deceased::main_search (
            params.first_name.as_deref().unwrap().to_string(),
            params.middle_name.clone(),
            params.last_name.as_deref().unwrap().to_string(),
            birth_date,
            death_date,
            location,
        );
        if user_id.is_some() {
            let _request_user = user_id.unwrap();

            #[derive(TemplateOnce)]
            #[template(path = "desctop/main/search.stpl")]
            struct Template {
                request_user: User,
                object_list:  Vec<Deceased>,
            }
            let body = Template {
                request_user: _request_user,
                object_list:  object_list,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/main/anon_search.stpl")]
            struct Template {
                object_list: Vec<Deceased>,
            }
            let body = Template {
                object_list: object_list,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
    else {
        return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("no params"));
    }
}


pub async fn image_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let _file = crate::utils::get_file(*_id).expect("E.");
    let mut _description = String::new();
    match _file.object_types {
        1 => {
            let _organization = crate::utils::get_organization(_file.object_id).expect("E.");
            _description == "Изображение организации ".to_string() + &_organization.name;
        },
        2 => {
            let _place = crate::utils::get_place(_file.object_id).expect("E.");
            _description == "Изображение кладбища ".to_string() + &_place.title;
        },
        3 => {
            let _deceased = crate::utils::get_deceased(_file.object_id).expect("E.");
            _description == "Изображение усопшего ".to_string() + &_deceased.get_full_name();
        },
        _ => (),
    };

    let (_prev, _next) = _file.get_prev_next_images();

    #[derive(TemplateOnce)]
    #[template(path = "desctop/load/image.stpl")]
    struct Template {
        file:        crate::models::File,
        description: String,
        prev:        Option<crate::models::File>,
        next:        Option<crate::models::File>,
    }
    let body = Template {
        file:        _file,
        description: _description,
        prev:        _prev,
        next:        _next,
    }
    .render_once()
    .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        
}

