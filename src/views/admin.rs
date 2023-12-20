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
    District, Citie, Region, Countrie,
};

pub fn admin_routes(config: &mut web::ServiceConfig) {
    config.route("/load_countries/", web::get().to(load_countries));
    config.route("/load_regions/{id}/", web::get().to(load_regions));
    config.route("/load_region_districts/{id}/", web::get().to(load_region_districts));
    config.route("/load_country_districts/{id}/", web::get().to(load_country_districts));
    config.route("/load_region_cities/{id}/", web::get().to(load_region_cities));
    config.route("/load_country_cities/{id}/", web::get().to(load_country_cities));
    config.route("/load_region_geo_items/{id}/", web::get().to(load_region_geo_items));

    config.route("/create_country/", web::get().to(create_country_page));
    config.route("/edit_country/{id}/", web::get().to(edit_country_page));
    config.route("/create_region/", web::get().to(create_region_page));
    config.route("/edit_region/{id}/", web::get().to(edit_region_page));
    config.route("/create_district/", web::get().to(create_district_page));
    config.route("/edit_district/{id}/", web::get().to(edit_district_page));
    config.route("/create_city/", web::get().to(create_city_page));
    config.route("/edit_city/{id}/", web::get().to(edit_city_page));

    config.route("/create_country/", web::post().to(create_country));
    config.route("/edit_country/{id}/", web::post().to(edit_country));
    config.route("/delete_country/", web::post().to(delete_country));
    config.route("/create_region/", web::post().to(create_region));
    config.route("/edit_region/{id}/", web::post().to(edit_region));
    config.route("/delete_region/", web::post().to(delete_region));
    config.route("/create_district/", web::post().to(create_district));
    config.route("/edit_district/{id}/", web::post().to(edit_district));
    config.route("/delete_district/", web::post().to(delete_district));
    config.route("/create_city/", web::post().to(create_city));
    config.route("/edit_city/{id}/", web::post().to(edit_city));
    config.route("/delete_region/", web::post().to(delete_city));

    config.route("/admin/organizations/", web::get().to(suggested_organizations_page));

    config.route("/users/all/", web::get().to(all_users_list));
    config.route("/users/create_admin/", web::post().to(create_admin));
    config.route("/users/remove_staff/", web::post().to(remove_staff));
}


pub async fn all_users_list(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_id = get_request_user(&req).await;
    if user_id.is_none() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = user_id.unwrap();
        let users_list = crate::models::User::get_all(_request_user.id);

        #[derive(TemplateOnce)] 
        #[template(path = "desctop/admin/users.stpl")]
        struct Template { 
            request_user: User,
            users_list:   Vec<User>,
        }
        let body = Template {
            request_user: _request_user,
            users_list:   users_list,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}
pub async fn load_countries(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_id = get_request_user(&req).await;
    if user_id.is_none() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = user_id.unwrap();
        let country_list = Countrie::get_all();

        #[derive(TemplateOnce)] 
        #[template(path = "desctop/load/load_countries.stpl")]
        struct Template { 
            country_list: Vec<Countrie>,
        }
        let body = Template {
            country_list: country_list,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}
pub async fn load_regions(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let user_id = get_request_user(&req).await;
    if user_id.is_none() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = user_id.unwrap();
        let region_list = Region::get_country_all(*_id);

        #[derive(TemplateOnce)]
        #[template(path = "desctop/load/load_regions.stpl")]
        struct Template { 
            region_list: Vec<Region>,
        }
        let body = Template {
            region_list: region_list,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}
pub async fn load_region_districts(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let user_id = get_request_user(&req).await;
    if user_id.is_none() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = user_id.unwrap();
        let district_list = District::get_region_all(*_id);

        #[derive(TemplateOnce)]
        #[template(path = "desctop/load/load_districts.stpl")]
        struct Template { 
            district_list: Vec<District>,
        }
        let body = Template {
            district_list: district_list,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}
pub async fn load_country_districts(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let user_id = get_request_user(&req).await;
    if user_id.is_none() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = user_id.unwrap();
        let district_list = District::get_country_all(*_id);

        #[derive(TemplateOnce)]
        #[template(path = "desctop/load/load_districts.stpl")]
        struct Template { 
            district_list: Vec<District>,
        }
        let body = Template {
            district_list: district_list,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}


pub async fn load_region_cities(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let user_id = get_request_user(&req).await;
    if user_id.is_none() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = user_id.unwrap();
        let city_list = Citie::get_region_all(*_id);

        #[derive(TemplateOnce)]
        #[template(path = "desctop/load/load_cities.stpl")]
        struct Template { 
            city_list: Vec<Citie>,
        }
        let body = Template {
            city_list: city_list,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}
pub async fn load_country_cities(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let user_id = get_request_user(&req).await;
    if user_id.is_none() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = user_id.unwrap();
        let city_list = Citie::get_country_all(*_id);

        #[derive(TemplateOnce)]
        #[template(path = "desctop/load/load_cities.stpl")]
        struct Template { 
            city_list: Vec<Citie>,
        }
        let body = Template {
            city_list: city_list,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}
pub async fn load_region_geo_items(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let user_id = get_request_user(&req).await;
    if user_id.is_none() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = user_id.unwrap();
        let city_list = Citie::get_region_all(*_id);
        let district_list = District::get_region_all(*_id);

        #[derive(TemplateOnce)]
        #[template(path = "desctop/load/load_districts_cities.stpl")]
        struct Template { 
            city_list:     Vec<Citie>,
            district_list: Vec<District>,
        }
        let body = Template {
            city_list:     city_list,
            district_list: district_list,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}


pub async fn create_country_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_id = get_request_user(&req).await;
    if user_id.is_none() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = user_id.unwrap();
        if !_request_user.is_admin() {
            return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("403"));
        }
        let country_list = Countrie::get_all();

        #[derive(TemplateOnce)]
        #[template(path = "desctop/admin/create_country.stpl")]
        struct Template { 
            request_user: User,
            country_list: Vec<Countrie>,
        }
        let body = Template {
            request_user: _request_user,
            country_list: country_list,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}
pub async fn edit_country_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let user_id = get_request_user(&req).await;
    if user_id.is_none() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = user_id.unwrap();
        if !_request_user.is_admin() {
            return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("403"));
        }
        let object = crate::utils::get_country(*_id).expect("E.");
        let country_list = Countrie::get_all();

        #[derive(TemplateOnce)]
        #[template(path = "desctop/admin/edit_country.stpl")]
        struct Template { 
            request_user: User,
            object:       Countrie,
            country_list: Vec<Countrie>,
        }
        let body = Template {
            request_user: _request_user,
            object:       object,
            country_list: country_list,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}

pub async fn create_region_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_id = get_request_user(&req).await;
    if user_id.is_none() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = user_id.unwrap();
        if !_request_user.is_admin() {
            return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("403"));
        }
        let country_list = Countrie::get_all();
        let region_list = Region::get_all();

        #[derive(TemplateOnce)]
        #[template(path = "desctop/admin/create_region.stpl")]
        struct Template { 
            request_user: User,
            country_list: Vec<Countrie>,
            region_list:  Vec<Region>,
        }
        let body = Template {
            request_user: _request_user,
            country_list: country_list,
            region_list:  region_list,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}
pub async fn edit_region_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let user_id = get_request_user(&req).await;
    if user_id.is_none() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = user_id.unwrap();
        if !_request_user.is_admin() {
            return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("403"));
        }
        let object = crate::utils::get_region(*_id).expect("E.");
        let country_list = Countrie::get_all();
        let region_list = Region::get_all();

        #[derive(TemplateOnce)]
        #[template(path = "desctop/admin/edit_region.stpl")]
        struct Template { 
            request_user: User,
            object:       Region,
            country_list: Vec<Countrie>,
            region_list:  Vec<Region>,
        }
        let body = Template {
            request_user: _request_user,
            object:       object,
            country_list: country_list,
            region_list:  region_list,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}

pub async fn create_district_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_id = get_request_user(&req).await;
    if user_id.is_none() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = user_id.unwrap();
        if !_request_user.is_admin() {
            return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("403"));
        }
        let country_list = Countrie::get_all();
        let district_list = District::get_all();

        #[derive(TemplateOnce)]
        #[template(path = "desctop/admin/create_district.stpl")]
        struct Template { 
            request_user:  User,
            country_list:  Vec<Countrie>,
            district_list: Vec<District>,
        }
        let body = Template {
            request_user:  _request_user,
            country_list:  country_list,
            district_list: district_list,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}
pub async fn edit_district_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let user_id = get_request_user(&req).await;
    if user_id.is_none() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = user_id.unwrap();
        if !_request_user.is_admin() {
            return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("403"));
        }
        let object = crate::utils::get_district(*_id).expect("E.");
        let country_list = Countrie::get_all();
        let region_list = Region::get_country_all(object.country_id);
        let district_list = District::get_all();

        #[derive(TemplateOnce)]
        #[template(path = "desctop/admin/edit_district.stpl")]
        struct Template { 
            request_user:  User,
            object:        District,
            country_list:  Vec<Countrie>,
            region_list:   Vec<Region>,
            district_list: Vec<District>,
        }
        let body = Template {
            request_user:  _request_user,
            object:        object,
            country_list:  country_list,
            region_list:   region_list,
            district_list: district_list,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}

pub async fn create_city_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_id = get_request_user(&req).await;
    if user_id.is_none() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = user_id.unwrap();
        if !_request_user.is_admin() {
            return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("403"));
        }
        let country_list = Countrie::get_all();
        let city_list = Citie::get_all();

        #[derive(TemplateOnce)]
        #[template(path = "desctop/admin/create_city.stpl")]
        struct Template { 
            request_user: User,
            country_list: Vec<Countrie>,
            city_list:    Vec<Citie>,
        }
        let body = Template {
            request_user: _request_user,
            country_list: country_list,
            city_list:    city_list,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}
pub async fn edit_city_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let user_id = get_request_user(&req).await;
    if user_id.is_none() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = user_id.unwrap();
        if !_request_user.is_admin() {
            return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("403"));
        }
        let object = crate::utils::get_city(*_id).expect("E.");
        let country_list = Countrie::get_all();
        let region_list = Region::get_country_all(object.country_id);
        let city_list = Citie::get_all();

        #[derive(TemplateOnce)]
        #[template(path = "desctop/admin/edit_city.stpl")]
        struct Template { 
            request_user: User,
            object:       Citie,
            country_list: Vec<Countrie>,
            region_list:  Vec<Region>,
            city_list:    Vec<Citie>,
        }
        let body = Template {
            request_user: _request_user,
            object:       object,
            country_list: country_list,
            region_list:  region_list,
            city_list:    city_list,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}


pub async fn create_district(req: HttpRequest, mut payload: Multipart) -> impl Responder {
    let _user = get_request_user(&req).await;
    if _user.is_some() {
        let _request_user = _user.unwrap();
        if _request_user.is_admin() {
            let form = crate::utils::district_form(payload.borrow_mut()).await;
            District::create (  
                form.region_id,
                form.country_id,
                form.name.clone(),
                form.lat,
                form.lon,
            );
        }
    };
    HttpResponse::Ok()
}
pub async fn edit_district(req: HttpRequest, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    let user_id = get_request_user(&req).await;
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        let _district = crate::utils::get_district(*_id).expect("E."); 
        if _request_user.is_admin() {
            let form = crate::utils::district_form(payload.borrow_mut()).await;
            _district.edit (
                form.region_id,
                form.country_id,
                form.name.clone(),
                form.lat,
                form.lon,
            );
        }
    };
    HttpResponse::Ok()
}                                                          
pub async fn delete_district(req: HttpRequest, mut payload: Multipart) -> impl Responder {
    let user_id = get_request_user(&req).await; 
    if user_id.is_some() { 
        let form = crate::utils::id_form(payload.borrow_mut()).await;
        let _request_user = user_id.unwrap();
        let _district = crate::utils::get_district(form.id).expect("E.");
        if _request_user.is_admin() {
            _district.delete();
        }
    };
    HttpResponse::Ok()
}


pub async fn create_city(req: HttpRequest, mut payload: Multipart) -> impl Responder {
    let _user = get_request_user(&req).await;
    if _user.is_some() {
        let _request_user = _user.unwrap();
        if _request_user.is_admin() {
            let form = crate::utils::district_form(payload.borrow_mut()).await;
            Citie::create (  
                form.region_id,
                form.country_id,
                form.name.clone(),
                form.lat,
                form.lon,
            );
        }
    }; 
    HttpResponse::Ok()
}
pub async fn edit_city(req: HttpRequest, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    let user_id = get_request_user(&req).await;
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        let _city = crate::utils::get_city(*_id).expect("E."); 
        if _request_user.is_admin() {
            let form = crate::utils::district_form(payload.borrow_mut()).await;
            _city.edit (
                form.region_id,
                form.country_id,
                form.name.clone(),
                form.lat,
                form.lon,
            );
        }
    };
    HttpResponse::Ok()
}
pub async fn delete_city(req: HttpRequest, mut payload: Multipart) -> impl Responder {
    let user_id = get_request_user(&req).await; 
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        let form = crate::utils::id_form(payload.borrow_mut()).await;
        let _city = crate::utils::get_city(form.id).expect("E.");
        if _request_user.is_admin() {
            _city.delete();
        }
    };
    HttpResponse::Ok()
}


pub async fn create_region(req: HttpRequest, mut payload: Multipart) -> impl Responder {
    let _user = get_request_user(&req).await;
    if _user.is_some() {
        let _request_user = _user.unwrap();
        if _request_user.is_admin() {
            let form = crate::utils::region_form(payload.borrow_mut()).await;
            Region::create (  
                form.country_id,
                form.name.clone(),
                form.lat,
                form.lon,
            );
        }
    }; 
    HttpResponse::Ok()
}
pub async fn edit_region(req: HttpRequest, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    let user_id = get_request_user(&req).await;
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        let _region = crate::utils::get_region(*_id).expect("E."); 
        if _request_user.is_admin() {
            let form = crate::utils::region_form(payload.borrow_mut()).await;
            _region.edit (
                form.country_id,
                form.name.clone(),
                form.lat,
                form.lon,
            );
        }
    };
    HttpResponse::Ok()
}
pub async fn delete_region(req: HttpRequest, mut payload: Multipart) -> impl Responder {
    let user_id = get_request_user(&req).await; 
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        let form = crate::utils::id_form(payload.borrow_mut()).await;
        let _region = crate::utils::get_region(form.id).expect("E.");
        if _request_user.is_admin() {
            _region.delete();
        }
    };
    HttpResponse::Ok()
}


pub async fn create_country(req: HttpRequest, mut payload: Multipart) -> impl Responder {
    let _user = get_request_user(&req).await;
    if _user.is_some() {
        let _request_user = _user.unwrap();
        if _request_user.is_admin() {
            let form = crate::utils::country_form(payload.borrow_mut()).await;
            Countrie::create (  
                form.name.clone(),
                form.lat,
                form.lon,
            );
        }
    }; 
    HttpResponse::Ok()
}
pub async fn edit_country(req: HttpRequest, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    let user_id = get_request_user(&req).await;
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        let _country = crate::utils::get_country(*_id).expect("E."); 
        if _request_user.is_admin() {
            let form = crate::utils::country_form(payload.borrow_mut()).await;
            _country.edit (
                form.name.clone(),
                form.lat,
                form.lon,
            );
        }
    };
    HttpResponse::Ok()
}
pub async fn delete_country(req: HttpRequest, mut payload: Multipart) -> impl Responder {
    let user_id = get_request_user(&req).await; 
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        let form = crate::utils::id_form(payload.borrow_mut()).await;
        let _country = crate::utils::get_country(form.id).expect("E.");
        if _request_user.is_admin() {
            _country.delete();
        }
    };
    HttpResponse::Ok()
}

pub async fn suggested_organizations_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_id = get_request_user(&req).await;
    if user_id.is_none() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        use crate::models::Organization;

        let _request_user = user_id.unwrap();
        if !_request_user.is_admin() {
            return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("403"));
        }
        let org_list = Organization::suggested();

        #[derive(TemplateOnce)]
        #[template(path = "desctop/admin/suggested_organizations.stpl")]
        struct Template { 
            request_user: User,
            org_list:     Vec<Organization>,
        }
        let body = Template {
            request_user: _request_user,
            org_list:     org_list,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}


pub async fn create_admin(req: HttpRequest, mut payload: Multipart) -> impl Responder {
    let user_id = get_request_user(&req).await; 
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        let form = crate::utils::id_form(payload.borrow_mut()).await;
        let _user = crate::utils::get_user(form.id).expect("E.");
        if _request_user.is_admin() {
            _user.create_admin();
        }
    };
    HttpResponse::Ok()
}
pub async fn remove_staff(req: HttpRequest, mut payload: Multipart) -> impl Responder {
    let user_id = get_request_user(&req).await; 
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        let form = crate::utils::id_form(payload.borrow_mut()).await;
        let _user = crate::utils::get_user(form.id).expect("E.");
        if _request_user.is_admin() {
            _user.remove_staff();
        }
    };
    HttpResponse::Ok()
}