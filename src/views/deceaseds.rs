use actix_web::{
    web,
    web::block,
    HttpRequest,
    HttpResponse,
    Responder,
    error::InternalError,
    http::StatusCode,
};
use crate::errors::Error;
use crate::models::{
    Deceased,
    Organization,
    Review,
    Service,
    User,
    Place,
};
use sailfish::TemplateOnce;
use diesel::{
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
    PgConnection,
    Connection,
};
use actix_multipart::Multipart;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use crate::schema;
use crate::utils::{
    establish_connection,
    get_request_user,
};
use std::borrow::BorrowMut;


pub fn deceased_routes(config: &mut web::ServiceConfig) {
    config.route("/places/{id}/deceased_list/", web::get().to(all_deceased_place_page));
    config.route("/deceased/{id}/", web::get().to(deceased_page));
    config.route("/create_deceased/", web::get().to(create_deceased_page));
    config.route("/edit_deceased/{id}/", web::get().to(edit_deceased_page));

    config.route("/create_deceased/", web::post().to(create_deceased));
    config.route("/edit_deceased/{id}/", web::post().to(edit_deceased));
    config.route("/delete_deceased/", web::post().to(delete_deceased));
}


pub async fn all_deceased_place_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);

    let _place = crate::utils::get_place(*_id).expect("E.");
    let user_id = get_request_user(&req).await;
    let page = crate::utils::get_page(&req);
    let count = Deceased::count(*_id);

    let mut next_page_number = 0;
    let have_next: i32;
    let object_list: Vec<Deceased>;

    if page > 1 {
        let step = (page - 1) * 20;
        have_next = page * 20 + 1;
        object_list = Deceased::list(*_id, 20, step.into());
    }
    else {
        have_next = 20 + 1;
        object_list = Deceased::list(*_id, 20, 0);
    }
    if count > (have_next as usize) {
        next_page_number = page + 1;
    }

    if user_id.is_some() { 
        let _request_user = user_id.unwrap();
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/deceased/all_deceased_place.stpl")]
            struct Template {
                request_user:     User,
                place:            Place,
                object_list:      Vec<Deceased>,
                next_page_number: i32,
                is_ajax:          i32,
            }
            let body = Template {
                request_user:     _request_user,
                place:            _place,
                object_list:      object_list,
                next_page_number: next_page_number,
                is_ajax:          is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/deceased/all_deceased_place.stpl")]
            struct Template {
                request_user:     User,
                place:            Place,
                object_list:      Vec<Deceased>,
                next_page_number: i32,
                is_ajax:          i32,
            }
            let body = Template {
                request_user:     _request_user,
                place:            _place,
                object_list:      object_list,
                next_page_number: next_page_number,
                is_ajax:          is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
    else {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/deceased/anon_all_deceased_place.stpl")]
            struct Template {
                place:            Place,
                object_list:      Vec<Deceased>,
                next_page_number: i32,
                is_ajax:          i32,
            }
            let body = Template {
                place:            _place,
                object_list:      object_list,
                next_page_number: next_page_number,
                is_ajax:          is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/deceased/anon_all_deceased_place.stpl")]
            struct Template {
                place:            Place,
                object_list:      Vec<Deceased>,
                next_page_number: i32,
                is_ajax:          i32,
            }
            let body = Template {
                place:            _place,
                object_list:      object_list,
                next_page_number: next_page_number,
                is_ajax:          is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn deceased_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
    let _deceased = crate::utils::get_deceased(*_id).expect("E.");
    let user_id = get_request_user(&req).await;
    if user_id.is_some() { 
        let _request_user = user_id.unwrap();
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/deceased/deceased.stpl")]
            struct Template {
                request_user: User,
                deceased:     Deceased,
                is_ajax:      i32,
            }
            let body = Template {
                request_user: _request_user,
                deceased:     _deceased,
                is_ajax:      is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/deceased/deceased.stpl")]
            struct Template {
                request_user: User,
                deceased:     Deceased,
                is_ajax:      i32,
            }
            let body = Template {
                request_user: _request_user,
                deceased:     _deceased,
                is_ajax:      is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
    else {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/deceased/anon_deceased.stpl")]
            struct Template {
                deceased: Deceased,
                is_ajax:  i32,
            }
            let body = Template {
                deceased: _deceased,
                is_ajax: is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/deceased/anon_deceased.stpl")]
            struct Template {
                deceased: Deceased,
                is_ajax:  i32,
            }
            let body = Template {
                deceased: _deceased,
                is_ajax:  is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn create_deceased_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
    let user_id = get_request_user(&req).await;
    if user_id.is_some() { 
        let _request_user = user_id.unwrap();
        let places_list = Place::get_all();
        let deceased_list = Deceased::get_all(); 

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/deceased/create_deceased.stpl")]
            struct Template {
                request_user:  User,
                places_list:   Vec<Place>,
                deceased_list: Vec<Deceased>,
                is_ajax:       i32,
            }
            let body = Template {
                request_user:  _request_user,
                places_list:   places_list,
                deceased_list: deceased_list,
                is_ajax:       is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/deceased/create_deceased.stpl")]
            struct Template {
                request_user:  User,
                places_list:   Vec<Place>,
                deceased_list: Vec<Deceased>,
                is_ajax:       i32,
            }
            let body = Template {
                request_user:  _request_user,
                places_list:   places_list,
                deceased_list: deceased_list,
                is_ajax:       is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
    else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("anon"))
    }
}

pub async fn edit_deceased_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
    let _deceased = crate::utils::get_deceased(*_id).expect("E.");
    let _place = crate::utils::get_place(_deceased.place_id).expect("E.");
    let user_id = get_request_user(&req).await;
    let place_list = Place::get_all();
    let deceased_list = Deceased::get_all(); 

    if user_id.is_some() { 
        let _request_user = user_id.unwrap();
        if !_request_user.is_admin() {
            return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("403"));
        }
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/deceased/edit_deceased.stpl")]
            struct Template {
                request_user:  User,
                deceased:      Deceased,
                place:         Place,
                is_ajax:       i32,
                place_list:    Vec<Place>,
                deceased_list: Vec<Deceased>,
            }
            let body = Template {
                request_user:  _request_user,
                deceased:      _deceased,
                place:         _place,
                is_ajax:       is_ajax,
                place_list:    place_list,
                deceased_list: deceased_list,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/deceased/edit_deceased.stpl")]
            struct Template {
                request_user:  User,
                deceased:      Deceased,
                place:         Place,
                is_ajax:       i32,
                place_list:    Vec<Place>,
                deceased_list: Vec<Deceased>,
            }
            let body = Template {
                request_user:  _request_user,
                deceased:      _deceased,
                place:         _place,
                is_ajax:       is_ajax,
                place_list:    place_list,
                deceased_list: deceased_list,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
    else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("anon"))
    }
}

pub async fn create_deceased(req: HttpRequest, mut payload: Multipart) -> impl Responder {
    let _user = get_request_user(&req).await;
    if _user.is_some() {
        let _request_user = _user.unwrap();

        let form = crate::utils::deceased_form(payload.borrow_mut()).await;
        Deceased::create ( 
            _request_user.id, 
            form.place_id,
            form.first_name.clone(),
            form.middle_name.clone(),
            form.last_name.clone(),
            form.birth_date.clone(),
            form.death_date.clone(),
            form.image.clone(),
            form.memory_words.clone(),
            form.lat.clone(),
            form.lon.clone(),
            form.images.clone(),
        );
    };
    HttpResponse::Ok()
}

pub async fn edit_deceased(req: HttpRequest, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    let user_id = get_request_user(&req).await;
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        let _deceased = crate::utils::get_deceased(*_id).expect("E."); 
        let form = crate::utils::deceased_form(payload.borrow_mut()).await;
        _deceased.edit (
            _request_user.id,
            form.first_name.clone(),
            form.middle_name.clone(),
            form.last_name.clone(),
            form.birth_date.clone(),
            form.death_date.clone(),
            form.image.clone(),
            form.memory_words.clone(),
            form.lat.clone(),
            form.lon.clone(),
            form.images.clone(),
        );
    };
    HttpResponse::Ok()
}
pub async fn delete_deceased(req: HttpRequest, mut payload: Multipart) -> impl Responder {
    let user_id = get_request_user(&req).await; 
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        let form = crate::utils::id_form(payload.borrow_mut()).await;
        let _deceased = crate::utils::get_deceased(form.id).expect("E.");
        if _request_user.id == _deceased.user_id || _request_user.is_admin() {
            _deceased.delete();
        }
    };
    HttpResponse::Ok()
}