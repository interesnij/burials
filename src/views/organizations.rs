use actix_web::{
    web,
    web::block,
    HttpRequest,
    HttpResponse,
    error::InternalError,
    http::StatusCode,
    Responder,
};
use crate::errors::Error;
use crate::models::{
    Organization,
    Review,
    Service,
    User,
    Countrie,
    PlaceSmall,
    OrganizationPlace,
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


pub fn organization_routes(config: &mut web::ServiceConfig) {
    config.route("/all_organization_city/{id}/", web::get().to(all_organization_city_page));
    config.route("/all_organization_region/{id}/", web::get().to(all_organization_region_page));
    config.route("/all_organization_country/{id}/", web::get().to(all_organization_country_page));
    config.route("/organization/{id}/", web::get().to(organization_page));

    config.route("/create_organization/", web::get().to(create_organization_page));
    config.route("/edit_organization/{id}/", web::get().to(edit_organization_page));
    config.route("/create_organization/", web::post().to(create_organization));
    config.route("/edit_organization/{id}/", web::post().to(edit_organization));
    config.route("/delete_organization/", web::post().to(delete_organization));

    config.route("/create_service/{id}/", web::get().to(create_service_page));
    config.route("/edit_service/{id}/", web::get().to(edit_service_page));
    config.route("/create_service/{id}/", web::post().to(create_service));
    config.route("/edit_service/{id}/", web::post().to(edit_service));
    config.route("/delete_service/", web::post().to(delete_service));

    config.route("/create_loc/{id}/", web::get().to(create_loc_page));
    config.route("/edit_loc/{id}/", web::get().to(edit_loc_page));
    config.route("/create_loc/{id}/", web::post().to(create_loc));
    config.route("/edit_loc/{id}/", web::post().to(edit_loc));
    config.route("/delete_loc/", web::post().to(delete_loc));
}


pub async fn all_organization_city_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let is_desctop = is_desctop(&req);
    let _city = crate::utils::get_city(*_id).expect("E.");
    let (_organizations, _places)  = block(move || Organization::get_city_organizations(_city.id)).await?;
    let user_id = get_request_user(&req).await;
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/organization/all_organization_city_page.stpl")]
            struct Template {
                request_user:      User,
                city:              Citie,
                all_organizations: Vec<Organization>,
                all_places:        Vec<PlaceSmall>,
            }
            let body = Template {
                request_user:      _request_user,
                city:              _city,
                all_organizations: _organizations,
                all_places:        all_places,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/organization/all_organization_city_page.stpl")]
            struct Template {
                request_user:      User,
                city:              Citie,
                all_organizations: Vec<Organization>,
                all_places:        Vec<PlaceSmall>,
            }
            let body = Template {
                request_user:      _request_user,
                city:              _city,
                all_organizations: _organizations,
                all_places:        all_places,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
    else {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/organization/anon_all_organization_city_page.stpl")]
            struct Template {
                request_user:     User,
                city:             Citie,
                all_organization: Vec<Organization>,
                all_places:       Vec<PlaceSmall>,
            }
            let body = Template {
                request_user:     _request_user,
                city:             _city,
                all_organization: _organization,
                all_places:       all_places,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/organization/anon_all_organization_city_page.stpl")]
            struct Template {
                request_user:     User,
                city:             Citie,
                all_organization: Vec<Organization>,
                all_places:       Vec<PlaceSmall>,
            }
            let body = Template {
                request_user:     _request_user,
                city:             _city,
                all_organization: _organization,
                all_places:       all_places,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

//Получение всех организаций одного региона
pub async fn all_organization_region_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let is_desctop = is_desctop(&req);
    let _region = crate::utils::get_region(*_id).expect("E.");
    let (_organizations, _places)  = block(move || Organization::get_region_organizations(_region.id)).await?;
    let user_id = get_request_user(&req).await;
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/organization/all_organization_region_page.stpl")]
            struct Template {
                request_user:      User,
                region:            Region,
                all_organizations: Vec<Organization>,
                all_places:        Vec<PlaceSmall>,
            }
            let body = Template {
                request_user:      _request_user,
                region:            _region,
                all_organizations: _organizations,
                all_places:        all_places,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/organization/all_organization_region_page.stpl")]
            struct Template {
                request_user:      User,
                region:            Region,
                all_organizations: Vec<Organization>,
                all_places:        Vec<PlaceSmall>,
            }
            let body = Template {
                request_user:      _request_user,
                region:            _region,
                all_organizations: _organizations,
                all_places:        all_places,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
    else {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/organization/anon_all_organization_region_page.stpl")]
            struct Template {
                region:            Region,
                all_organizations: Vec<Organization>,
                all_places:        Vec<PlaceSmall>,
            }
            let body = Template {
                region:            _region,
                all_organizations: _organizations,
                all_places:        all_places,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/organization/anon_all_organization_region_page.stpl")]
            struct Template {
                region:            Region,
                all_organizations: Vec<Organization>,
                all_places:        Vec<PlaceSmall>,
            }
            let body = Template {
                region:            _region,
                all_organizations: _organizations,
                all_places:        all_places,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

//Получение всех организаций одной страны
pub async fn all_organization_countries_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let is_desctop = is_desctop(&req);
    let _country = crate::utils::get_country(*_id).expect("E.");
    let (_organizations, _places)  = block(move || Organization::get_country_organizations(_country.id)).await?;
    let user_id = get_request_user(&req).await;
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/organization/all_organization_countries_page.stpl")]
            struct Template {
                request_user:     User,
                country:          Countries,
                all_organization: Vec<Organization>,
                all_places:       Vec<PlaceSmall>,
            }
            let body = Template {
                request_user:     _request_user,
                country:          _countries,
                all_organization: _organizations,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/organization/all_organization_countries_page.stpl")]
            struct Template {
                request_user:     User,
                country:          Countries,
                all_organization: Vec<Organization>,
                all_places:       Vec<PlaceSmall>,
            }
            let body = Template {
                request_user:     _request_user,
                country:          _countries,
                all_organization: _organizations,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
    else {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/organization/anon_all_organization_countries_page.stpl")]
            struct Template {
                request_user:     User,
                country:          Countries,
                all_organization: Vec<Organization>,
                all_places:       Vec<PlaceSmall>,
            }
            let body = Template {
                request_user:     _request_user,
                country:          _countries,
                all_organization: _organization,
                all_places:       Vec<PlaceSmall>,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/organization/anon_all_organization_countries_page.stpl")]
            struct Template {
                request_user:     User,
                country:          Countries,
                all_organization: Vec<Organization>,
                all_places:       Vec<PlaceSmall>,
            }
            let body = Template {
                request_user:     _request_user,
                country:          _country,
                all_organization: _organization,
                all_places:       all_places,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn organization_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let is_desctop = is_desctop(&req);
    let _organization = crate::utils::get_organization(*_id).expect("E.");
    let user_id = get_request_user(&req).await;
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/organization/organization_page.stpl")]
            struct Template {
                request_user: User,
                organization: Organization,
            }
            let body = Template {
                request_user: _request_user,
                organization: _organization,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/organization/organization_page.stpl")]
            struct Template {
                request_user: User,
                organization: Organization,
            }
            let body = Template {
                request_user: _request_user,
                organization: _organization,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
    else {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/organization/anon_organization_page.stpl")]
            struct Template {
                request_user:   User,
                organization:       Organization,
            }
            let body = Template {
                request_user:   _request_user,
                organization:       _organization,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/organization/anon_organization_page.stpl")]
            struct Template {
                request_user:   User,
                organization:       Organization,
            }
            let body = Template {
                request_user:   _request_user,
                organization:       _organization,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}


pub async fn create_organization_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
    let user_id = get_request_user(&req).await;
    if user_id.is_some() { 
        let _request_user = user_id.unwrap();
        let country_list = Countrie::get_all();

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/organization/create_organization.stpl")]
            struct Template {
                request_user: User,
                is_ajax:      i32,
                country_list: Vec<Countrie>,
            }
            let body = Template {
                request_user:  _request_user,
                is_ajax:       is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/organization/create_organization.stpl")]
            struct Template {
                request_user: User,
                is_ajax:      i32,
                country_list: Vec<Countrie>,
            }
            let body = Template {
                request_user: _request_user,
                is_ajax:      is_ajax,
                country_list: country_list,
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

pub async fn edit_organization_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
    let _organization = crate::utils::get_organization(*_id).expect("E.");
    let user_id = get_request_user(&req).await;

    if user_id.is_some() { 
        let _request_user = user_id.unwrap();
        let country_list = Countrie::get_all();
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/organization/edit_organization.stpl")]
            struct Template {
                request_user: User,
                organization: Organization,
                is_ajax:      i32,
                country_list: Vec<Countrie>,
            }
            let body = Template {
                request_user: _request_user,
                organization: _organization,
                is_ajax:      is_ajax,
                country_list: country_list,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/organization/edit_organization.stpl")]
            struct Template {
                request_user: User,
                organization: Organization,
                is_ajax:      i32,
                country_list: Vec<Countrie>,
            }
            let body = Template {
                request_user: _request_user,
                organization: _organization,
                is_ajax:      is_ajax,
                country_list: country_list,
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

pub async fn create_organization(req: HttpRequest, mut payload: Multipart) -> impl Responder {
    let _user = get_request_user(&req).await;
    if _user.is_some() { 
        let _request_user = _user.unwrap();
        let form = crate::utils::organization_form(payload.borrow_mut()).await;
        Organization::create ( 
            _request_user.id, 
            form.name.clone(),
            form.description.clone(),
            form.director.clone(),
            form.phone.clone(),
            form.hours.clone(),
            form.website.clone(),
            form.image.clone(),
        );
    }; 
    HttpResponse::Ok()
}

pub async fn edit_organization(req: HttpRequest, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    let user_id = get_request_user(&req).await;
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        let _organization = crate::utils::get_organization(*_id).expect("E."); 
        let form = crate::utils::organization_form(payload.borrow_mut()).await;
        _organization.edit (
            _request_user.id,
            form.name.clone(),
            form.description.clone(),
            form.director.clone(),
            form.phone.clone(),
            form.hours.clone(),
            form.website.clone(),
            form.image.clone(),
        );
    };
    HttpResponse::Ok()
}
pub async fn delete_organization(req: HttpRequest, mut payload: Multipart) -> impl Responder {
    let user_id = get_request_user(&req).await; 
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        let form = crate::utils::id_form(payload.borrow_mut()).await;
        let _organization = crate::utils::get_organization(form.id).expect("E.");
        if _request_user.id == _organization.user_id || _request_user.is_admin() {
            _organization.delete(_request_user.id);
        }
    };
    HttpResponse::Ok()
}


pub async fn create_service_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
    let user_id = get_request_user(&req).await;
    if user_id.is_some() { 
        let _request_user = user_id.unwrap();
        let _organization = crate::utils::get_organization(*_id).expect("E."); 
        if _request_user.id == _organization.user_id || _request_user.is_admin() {
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/service/create_service.stpl")]
                struct Template {
                    request_user: User,
                    is_ajax:      i32,
                    organization: Organization,
                }
                let body = Template {
                    request_user: _request_user,
                    is_ajax:      is_ajax,
                    organization: _organization,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/service/create_service.stpl")]
                struct Template {
                    request_user: User,
                    is_ajax:      i32,
                    organization: Organization,
                }
                let body = Template {
                    request_user: _request_user,
                    is_ajax:      is_ajax,
                    organization: _organization,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
        else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("403"))
        }
    }
    else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("anon"))
    }
}

pub async fn edit_service_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
    let _service = crate::utils::get_service(*_id).expect("E.");
    let _organization = crate::utils::get_organization(_service.organization_id).expect("E.");
    let user_id = get_request_user(&req).await;

    if user_id.is_some() { 
        let _request_user = user_id.unwrap();
        if _request_user.id == _organization.user_id || _request_user.is_admin() {
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/service/edit_service.stpl")]
                struct Template {
                    request_user: User,
                    organization: Organization,
                    is_ajax:      i32,
                    service:      Service,
                }
                let body = Template {
                    request_user: _request_user,
                    organization: _organization,
                    is_ajax:      is_ajax,
                    service:      _service,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/service/edit_service.stpl")]
                struct Template {
                    request_user: User,
                    organization: Organization,
                    is_ajax:      i32,
                    service:      Service,
                }
                let body = Template {
                    request_user: _request_user,
                    organization: _organization,
                    is_ajax:      is_ajax,
                    service:      _service,
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
    else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("anon"))
    }
}

pub async fn create_service(req: HttpRequest, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    let _user = get_request_user(&req).await;
    if _user.is_some() { 
        let _request_user = _user.unwrap();
        let form = crate::utils::service_form(payload.borrow_mut()).await;
        Service::create ( 
            _request_user.id,
            *_id,
            form.title.clone(),
            form.description.clone(),
            form.image.clone(),
            form.price,
        );
    }; 
    HttpResponse::Ok()
}

pub async fn edit_service(req: HttpRequest, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    let user_id = get_request_user(&req).await;
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        let _service = crate::utils::get_service(*_id).expect("E.");
        let form = crate::utils::service_form(payload.borrow_mut()).await;
        _service.edit (
            _request_user.id,
            form.title.clone(),
            form.description.clone(),
            form.image.clone(),
            form.price,
        );
    }; 
    HttpResponse::Ok()
}
pub async fn delete_service(req: HttpRequest, mut payload: Multipart) -> impl Responder {
    let user_id = get_request_user(&req).await; 
    if user_id.is_some() {
        //let _request_user = user_id.unwrap();
        let form = crate::utils::id_form(payload.borrow_mut()).await;
        let _service = crate::utils::get_service(form.id).expect("E.");
        _service.delete(_request_user.id);
    }
    HttpResponse::Ok()
}


pub async fn create_loc_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
    let user_id = get_request_user(&req).await;
    if user_id.is_some() { 
        let _request_user = user_id.unwrap();
        let _organization = crate::utils::get_organization(*_id).expect("E."); 
        if _request_user.id == _organization.user_id || _request_user.is_admin() {
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/organization/create_loc.stpl")]
                struct Template {
                    request_user: User,
                    is_ajax:      i32,
                    organization: Organization,
                }
                let body = Template {
                    request_user: _request_user,
                    is_ajax:      is_ajax,
                    organization: _organization,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/organization/create_loc.stpl")]
                struct Template {
                    request_user: User,
                    is_ajax:      i32,
                    organization: Organization,
                }
                let body = Template {
                    request_user: _request_user,
                    is_ajax:      is_ajax,
                    organization: _organization,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
        else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("403"))
        }
    }
    else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("anon"))
    }
}

pub async fn edit_loc_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
    let _loc = crate::utils::get_organization_loc(*_id).expect("E.");
    let _organization = crate::utils::get_organization(_loc.organization_id).expect("E.");
    let user_id = get_request_user(&req).await;

    if user_id.is_some() { 
        let _request_user = user_id.unwrap();
        if _request_user.id == _organization.user_id || _request_user.is_admin() {
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/organization/edit_loc.stpl")]
                struct Template {
                    request_user: User,
                    organization: Organization,
                    is_ajax:      i32,
                    loc:          OrganizationPlace,
                }
                let body = Template {
                    request_user: _request_user,
                    organization: _organization,
                    is_ajax:      is_ajax,
                    loc:          loc,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/organization/edit_loc.stpl")]
                struct Template {
                    request_user: User,
                    organization: Organization,
                    is_ajax:      i32,
                    loc:          OrganizationPlace,
                }
                let body = Template {
                    request_user: _request_user,
                    organization: _organization,
                    is_ajax:      is_ajax,
                    loc:          loc,
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
    else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("anon"))
    }
}

pub async fn create_loc(req: HttpRequest, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    let _user = get_request_user(&req).await;
    if _user.is_some() { 
        let _request_user = _user.unwrap();
        let form = crate::utils::loc_form(payload.borrow_mut()).await;
        OrganizationPlace::create ( 
            _request_user.id,
            *_id, 
            form.city_id,
            form.region_id,
            form.country_id,
            form.address2.clone(),
        );
    }; 
    HttpResponse::Ok()
}

pub async fn edit_loc(req: HttpRequest, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    let user_id = get_request_user(&req).await;
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        let _loc = crate::utils::get_organization_loc(*_id).expect("E.");
        let form = crate::utils::loc_form(payload.borrow_mut()).await;
        _loc.edit (
            _request_user.id,
            form.address2,
        );
    }; 
    HttpResponse::Ok()
}
pub async fn delete_loc(req: HttpRequest, mut payload: Multipart) -> impl Responder {
    let user_id = get_request_user(&req).await; 
    if user_id.is_some() {
        //let _request_user = user_id.unwrap();
        let form = crate::utils::id_form(payload.borrow_mut()).await;
        let _loc = crate::utils::get_organization_loc(form.id).expect("E.");
        _loc.delete(_request_user.id);
    } 
    HttpResponse::Ok()
}