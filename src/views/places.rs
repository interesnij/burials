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
use serde::{Deserialize, Serialize};
use crate::schema;
use crate::utils::{
    establish_connection,
    get_request_user,
};
use std::borrow::BorrowMut;
use crate::models::{
    Place, District, Citie, Region, Countrie,
};


//-------------------------------------------------------------------------

pub fn place_routes(config: &mut web::ServiceConfig) {
    config.route("/places/", web::get().to(all_places_page));
    config.route("/all_place_city/{id}/", web::get().to(all_place_city_page));
    config.route("/all_place_region/{id}/", web::get().to(all_place_region_page));
    config.route("/all_place_countries/{id}/", web::get().to(all_place_countries_page));
    config.route("/place/{id}/", web::get().to(place_page));
    config.route("/create_place/", web::get().to(create_place_page));
    config.route("/edit_place/{id}/", web::get().to(edit_place_page));

    config.route("/create_place/", web::post().to(create_place));
    config.route("/edit_place/{id}/", web::post().to(edit_place));
    config.route("/delete_place/{id}/", web::post().to(delete_place));

}


pub async fn all_places_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);

    let user_id = get_request_user(&req).await;
    let object_list = Place::get_all();

    if user_id.is_some() { 
        let _request_user = user_id.unwrap();
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/places/all_places.stpl")]
            struct Template {
                request_user: User,
                object_list:  Vec<Place>,
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
            #[template(path = "mobile/places/all_places.stpl")]
            struct Template {
                request_user: User,
                object_list:  Vec<Place>,
            }
            let body = Template {
                request_user: _request_user,
                object_list:  object_list,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
    else {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/places/anon_all_places.stpl")]
            struct Template {
                object_list:  Vec<Place>,
            }
            let body = Template {
                object_list:  object_list,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/places/anon_all_places.stpl")]
            struct Template {
                object_list:  Vec<Place>,
            }
            let body = Template {
                object_list:  object_list,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn all_place_city_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
    let _city = crate::utils::get_city(*_id).expect("E."); 
    let _places = block(move || Place::city_list(*_id)).await?;
    let user_id = get_request_user(&req).await;
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/place/all_place_city.stpl")]
            struct Template {
                request_user: User,
                city:         Citie,
                all_places:   Vec<Place>,
            }
            let body = Template {
                request_user: _request_user,
                city:         _city,
                all_places:   _places,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/place/all_place_city.stpl")]
            struct Template {
                request_user: User,
                city:         Citie,
                all_places:   Vec<Place>,
            }
            let body = Template {
                request_user: _request_user,
                city:         _city,
                all_places:   _places,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
    else {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/place/anon_all_place_city.stpl")]
            struct Template {
                city:         Citie,
                all_places:   Vec<Place>,
            }
            let body = Template {
                city:         _city,
                all_places:   _places,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/place/anon_all_place_city.stpl")]
            struct Template {
                city:         Citie,
                all_places:   Vec<Place>,
            }
            let body = Template {
                city:         _city,
                all_places:   _places,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}
//Получение всех кладбищ одного региона

pub async fn all_place_region_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
    let _region = crate::utils::get_region(*_id).expect("E."); 
    let _places = block(move || Place::region_list(*_id)).await?;
    let user_id = get_request_user(&req).await;
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        if is_desctop { 
            #[derive(TemplateOnce)]
            #[template(path = "desctop/place/all_place_region.stpl")]
            struct Template {
                request_user: User,
                region:       Region,
                all_places:   Vec<Place>,
            }
            let body = Template {
                request_user: _request_user,
                region:       _region,
                all_places:   _places,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/place/all_place_region.stpl")]
            struct Template {
                request_user: User,
                region:       Region,
                all_places:   Vec<Place>,
            }
            let body = Template {
                request_user: _request_user,
                region:       _region,
                all_places:   _places,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
    else {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/place/anon_all_place_region.stpl")]
            struct Template {
                region:       Region,
                all_places:   Vec<Place>,
            }
            let body = Template {
                region:       _region,
                all_places:   _places,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/place/anon_all_place_region.stpl")]
            struct Template {
                region:       Region,
                all_places:   Vec<Place>,
            }
            let body = Template {
                region:       _region,
                all_places:   _places,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}
//Получение всех кладбищ одной страны

pub async fn all_place_countries_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
    let _country = crate::utils::get_country(*_id).expect("E."); 
    let _places = block(move || Place::country_list(*_id)).await?;
    let user_id = get_request_user(&req).await;
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/place/all_place_countries.stpl")]
            struct Template {
                request_user: User,
                country:      Countrie,
                all_places:   Vec<Place>,
            }
            let body = Template {
                request_user: _request_user,
                country:      _country,
                all_places:   _places,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/place/all_place_countries.stpl")]
            struct Template {
                request_user: User,
                country:      Countrie,
                all_places:   Vec<Place>,
            }
            let body = Template {
                request_user: _request_user,
                country:      _country,
                all_places:   _places,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
    else {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/place/anon_all_place_countries.stpl")]
            struct Template {
                country:      Countrie,
                all_places:   Vec<Place>,
            }
            let body = Template {
                country:      _country,
                all_places:   _places,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/place/anon_all_place_countries.stpl")]
            struct Template {
                country:      Countrie,
                all_places:   Vec<Place>,
            }
            let body = Template {
                country:      _country,
                all_places:   _places,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}
//-------------------------------------------------------------------------
//Получение кладбища 


pub async fn place_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
    let _place = crate::utils::get_place(*_id).expect("E."); 
    let user_id = get_request_user(&req).await;
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/place/place.stpl")]
            struct Template {
                request_user: User,
                place:        Place,
            }
            let body = Template {
                request_user: _request_user,
                place:        _place,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            
            #[derive(TemplateOnce)]
            #[template(path = "mobile/place/place.stpl")]
            struct Template {
                request_user: User,
                place:        Place,
            }
            let body = Template {
                request_user: _request_user,
                place:        _place,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
    else {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/place/anon_place.stpl")]
            struct Template {
                place:        Place,
            }
            let body = Template {
                place:        _place,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/place/anon_place.stpl")]
            struct Template {
                place:        Place,
            }
            let body = Template {
                place:        _place,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}


pub async fn create_place(req: HttpRequest, mut payload: Multipart) -> impl Responder {
    let _user = get_request_user(&req).await;
    if _user.is_some() {
        let _request_user = _user.unwrap();
        if _request_user.is_admin() {
            let form = crate::utils::place_form(payload.borrow_mut()).await;
            Place::create ( 
                _request_user.id,
                form.city_id,
                form.district_id,
                form.region_id,
                form.country_id,
                form.title.clone(),
                form.description.clone(),
                form.hours.clone(),
                form.image.clone(),
                form.address.clone(),
                form.director.clone(),
                form.phone.clone(),
                form.lat,
                form.lon,
            );
        }
    }; 
    HttpResponse::Ok()
}

pub async fn edit_place(req: HttpRequest, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    let user_id = get_request_user(&req).await;
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        let _place = crate::utils::get_place(*_id).expect("E."); 
        if _request_user.id == _place.user_id || _request_user.is_admin() {
            let form = crate::utils::place_form(payload.borrow_mut()).await;
            _place.edit ( 
                _request_user.id,
                form.city_id,
                form.district_id,
                form.region_id,
                form.country_id,
                form.title.clone(),
                form.description.clone(),
                form.hours.clone(),
                form.image.clone(),
                form.address.clone(),
                form.director.clone(),
                form.phone.clone(),
                form.lat,
                form.lon,
            );
        }
    };
    HttpResponse::Ok()
}
pub async fn delete_place(req: HttpRequest, _id: web::Path<i32>) -> impl Responder {
    let user_id = get_request_user(&req).await; 
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        let _place = crate::utils::get_place(*_id).expect("E.");
        if _request_user.id == _place.user_id || _request_user.is_admin() {
            _place.delete();
        }
    };
    HttpResponse::Ok()
}


pub async fn create_place_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
    let user_id = get_request_user(&req).await;
    if user_id.is_some() { 
        let _request_user = user_id.unwrap();
        if !_request_user.is_admin() {
            return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("403"));
        }

        let country_list = crate::models::Countrie::get_all();
        let place_list = crate::models::Place::get_all();

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/place/create_place.stpl")]
            struct Template {
                request_user: User,
                is_ajax:      i32,
                country_list: Vec<Countrie>,
                place_list:   Vec<Place>,
            }
            let body = Template {
                request_user: _request_user,
                is_ajax:      is_ajax,
                country_list: country_list,
                place_list:   place_list,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/place/create_place.stpl")]
            struct Template {
                request_user: User,
                is_ajax:      i32,
                country_list: Vec<Countrie>,
                place_list:   Vec<Place>,
            }
            let body = Template {
                request_user: _request_user,
                is_ajax:      is_ajax,
                country_list: country_list,
                place_list:   place_list,
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

pub async fn edit_place_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
    let _place = crate::utils::get_place(*_id).expect("E.");
    let user_id = get_request_user(&req).await;

    let country_list = Countrie::get_all();
    let region_list = Region::get_country_all(_place.country_id);
    let city_list = Citie::get_all(); 
    let place_list = crate::models::Place::get_all();

    if user_id.is_some() { 
        let _request_user = user_id.unwrap();
        if !_request_user.is_admin() {
            return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("403"));
        }
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/place/edit_place.stpl")]
            struct Template {
                request_user: User,
                place:        Place,
                is_ajax:      i32,
                country_list: Vec<Countrie>,
                region_list:  Vec<Region>,
                city_list:    Vec<Citie>,
                place_list:   Vec<Place>,
            }
            let body = Template {
                request_user: _request_user,
                place:        _place,
                is_ajax:      is_ajax,
                country_list: country_list,
                region_list:  region_list,
                city_list:    city_list,
                place_list:   place_list,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/place/edit_place.stpl")]
            struct Template {
                request_user: User,
                place:        Place,
                is_ajax:      i32,
                country_list: Vec<Countrie>,
                region_list:  Vec<Region>,
                city_list:    Vec<Citie>,
                place_list:   Vec<Place>,
            }
            let body = Template {
                request_user: _request_user,
                place:        _place,
                is_ajax:      is_ajax,
                country_list: country_list,
                region_list:  region_list,
                city_list:    city_list,
                place_list:   place_list,
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