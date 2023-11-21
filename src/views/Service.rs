
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
    Deceased,
    Organization,
    Rewiew,
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
use serde::{Deserialize, Serialize};
use crate::schema;
use crate::utils::{
    establish_connection,
    get_request_user,
};
use std::borrow::BorrowMut;



pub fn service_routes(config: &mut web::ServiceConfig) {

    config.route("/organization/{id}/services/", web::get().to(organization_services_page));
    config.route("/all_service_place/{id}/", web::get().to(all_service_place_page));
    config.route("/all_service_city/{id}/", web::get().to(all_service_city_page));
    config.route("/all_service_region/{id}/", web::get().to(all_service_region_page));
    config.route("/all_service_country/{id}/", web::get().to(all_service_country_page));
    config.route("/service/{id}/", web::get().to(service_page));
    config.route("/create_service/", web::get().to(create_service_page));
    config.route("/edit_service/{id}/", web::get().to(edit_service_page));

    config.route("/delete_service/{id}/", web::post().to(delete_service_page));
    config.route("/edit_service/{id}/", web::post().to(edit_service));
    config.route("/delete_service/{id}/", web::post().to(delete_service));

}

//Получение всех услуг одной организации
pub async fn organization_services_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let is_desctop = is_desctop(&req);
    let _organization = block(move || Organization::find_by_id(*_id)).await?;
    let _service = block(move || Service::get_all_service()).await?;
    let user_id = get_request_user(&req).await;
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/service/all_service_organization_page.stpl")]
            struct Template {
                request_user:   User,
                organization:          Vec<Organization>,
                all_service:  Vec<Service>,
            }
            let body = Template {
                request_user:   _request_user,
                organization:           _organization,
                all_service:   _service,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/service/all_service_organization_page.stpl")]
            struct Template {
                request_user:   User,
                organization:          Vec<Organization>,
                all_service:  Vec<Service>,
            }
            let body = Template {
                request_user:   _request_user,
                organization:           _organization,
                all_service:   _service,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
    else {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/service/anon_all_service_organization_page.stpl")]
            struct Template {
                request_user:   User,
                organization:          Vec<Organization>,
                all_service:  Vec<Service>,
            }
            let body = Template {
                request_user:   _request_user,
                organization:           _organization,
                all_service:   _service,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/service/anon_all_service_organization_page.stpl")]
            struct Template {
                request_user:   User,
                organization:           Vec<Organization>,
                all_service:       Vec<Service>,
            }
            let body = Template {
                request_user:   _request_user,
                organization:          _organization,
                all_service:  _service,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

//-------------------------------------------------------------------------

//Получение всех Услуг одного кладбища
pub async fn all_service_place_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let is_desctop = is_desctop(&req);
    let _place = block(move || Place::find_by_id(*_id)).await?;
    let _service = block(move || Service::get_all_service()).await?;
    let user_id = get_request_user(&req).await;
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/service/all_service_place_page.stpl")]
            struct Template {
                request_user:   User,
                place:          Vec<Place>,
                all_service:  Vec<Service>,
            }
            let body = Template {
                request_user:   _request_user,
                place:           _place,
                all_service:   _service,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/service/all_service_place_page.stpl")]
            struct Template {
                request_user:   User,
                place:          Vec<Place>,
                all_service:  Vec<Service>,
            }
            let body = Template {
                request_user:   _request_user,
                place:           _place,
                all_service:   _service,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
    else {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/service/anon_all_service_place_page.stpl")]
            struct Template {
                request_user:   User,
                place:          Vec<Place>,
                all_service:  Vec<Service>,
            }
            let body = Template {
                request_user:   _request_user,
                place:           _place,
                all_service:   _service,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/service/anon_all_service_place_page.stpl")]
            struct Template {
                request_user:   User,
                place:           Vec<Place>,
                all_service:       Vec<Service>,
            }
            let body = Template {
                request_user:   _request_user,
                place:          _place,
                all_service:  _service,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

//Получение всех Услуг одного города
pub async fn all_service_city_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let is_desctop = is_desctop(&req);
    let _city = block(move || City::find_by_id(*_id)).await?;
    let _organization = block(move || Service::get_all_service()).await?;
    let user_id = get_request_user(&req).await;
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/service/all_service_city_page.stpl")]
            struct Template {
                request_user:   User,
                city:          Vec<City>,
                all_service:  Vec<Service>,
            }
            let body = Template {
                request_user:   _request_user,
                city:           _city,
                all_service:   _service,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/service/all_service_city_page.stpl")]
            struct Template {
                request_user:   User,
                city:          Vec<City>,
                all_service:  Vec<Service>,
            }
            let body = Template {
                request_user:   _request_user,
                city:           _city,
                all_service:   _service,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
    else {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/service/anon_all_service_city_page.stpl")]
            struct Template {
                request_user:   User,
                city:          Vec<City>,
                all_service:  Vec<Service>,
            }
            let body = Template {
                request_user:   _request_user,
                city:           _city,
                all_service:   _service,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/service/anon_all_service_city_page.stpl")]
            struct Template {
                request_user:   User,
                city:           Vec<City>,
                all_service:       Vec<Service>,
            }
            let body = Template {
                request_user:   _request_user,
                city:          _city,
                all_service:  _service,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

//Получение всех Услуг одного региона
pub async fn all_service_region_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let is_desctop = is_desctop(&req);
    let _region = block(move || Region::find_by_id(*_id)).await?;
    let _service = block(move || Service::get_all_service()).await?;
    let user_id = get_request_user(&req).await;
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/service/all_service_region_page.stpl")]
            struct Template {
                request_user:   User,
                region:          Vec<Region>,
                all_service:  Vec<Service>,
            }
            let body = Template {
                request_user:   _request_user,
                region:           _region,
                all_service:   _service,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/service/all_service_region_page.stpl")]
            struct Template {
                request_user:   User,
                region:          Vec<Region>,
                all_service:  Vec<Service>,
            }
            let body = Template {
                request_user:   _request_user,
                region:           _region,
                all_service:   _service,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
    else {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/service/anon_all_service_region_page.stpl")]
            struct Template {
                request_user:   User,
                region:          Vec<Region>,
                all_service:  Vec<Service>,
            }
            let body = Template {
                request_user:   _request_user,
                region:           _region,
                all_service:   _service,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/service/anon_all_service_region_page.stpl")]
            struct Template {
                request_user:   User,
                region:           Vec<Region>,
                all_service:       Vec<Service>,
            }
            let body = Template {
                request_user:   _request_user,
                region:          _region,
                all_service:  _service,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

//Получение всех Услуг  одной страны
pub async fn all_service_countries_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let is_desctop = is_desctop(&req);
    let _countries = block(move || Countries::find_by_id(*_id)).await?;
    let _service = block(move || Service::get_all_service()).await?;
    let user_id = get_request_user(&req).await;
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/service/all_service_countries_page.stpl")]
            struct Template {
                request_user:   User,
                countries:          Vec<Countries>,
                all_service:  Vec<Service>,
            }
            let body = Template {
                request_user:   _request_user,
                countries:           _countries,
                all_service:   _service,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/service/all_service_countries_page.stpl")]
            struct Template {
                request_user:   User,
                countries:          Vec<Countries>,
                all_service:  Vec<Service>,
            }
            let body = Template {
                request_user:   _request_user,
                countries:           _countries,
                all_service:   _service,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
    else {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/service/anon_all_service_countries_page.stpl")]
            struct Template {
                request_user:   User,
                countries:          Vec<Countries>,
                all_service:  Vec<Service>,
            }
            let body = Template {
                request_user:   _request_user,
                countries:           _countries,
                all_service:   _service,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/service/anon_all_service_countries_page.stpl")]
            struct Template {
                request_user:   User,
                countries:           Vec<Countries>,
                all_service:       Vec<Service>,
            }
            let body = Template {
                request_user:   _request_user,
                countries:          _countries,
                all_service:  _service,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}
//-------------------------------------------------------------------------

//Получение Услуги по id
pub async fn service_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let is_desctop = is_desctop(&req);
    let _service = block(move || Service::find_by_id(*_id)).await?;
    let user_id = get_request_user(&req).await;
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/service/service_page.stpl")]
            struct Template {
                request_user:   User,
                service:       Service,
            }
            let body = Template {
                request_user:   _request_user,
                service:      _service,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/service/service_page.stpl")]
            struct Template {
                request_user:   User,
                service:       Service,
            }
            let body = Template {
                request_user:   _request_user,
                service:       _service,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
    else {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/service/anon_service_page.stpl")]
            struct Template {
                request_user:   User,
                service:       Service,
            }
            let body = Template {
                request_user:   _request_user,
                service:       _service,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/service/anon_service_page.stpl")]
            struct Template {
                request_user:   User,
                service:       Service,
            }
            let body = Template {
                request_user:   _request_user,
                service:       _service,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

//-------------------------------------------------------------------------

pub async fn create_service_page(req: HttpRequest, mut payload: Multipart) -> impl Responder {
    let user_id = get_request_user(&req).await;
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        if _request_user.is_admin() {
            let form = organization_form(payload.borrow_mut(), _request_user.id).await;
            Service::create_service(form);
        }
    };
    HttpResponse::Ok()
}

pub async fn edit_service_page(req: HttpRequest, _id: web::Path<i32>) -> impl Responder {
    let user_id = get_request_user(&req).await;
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        let _service = block(move || Service::find_by_id(*_id)).await?; 
        if _request_user.id == place.user_id {
            let form = organization_form(payload.borrow_mut(), _request_user.id).await;
            Service.edit_service(form);
        }
    };
    HttpResponse::Ok()
}
pub async fn delete_service_page(req: HttpRequest, _id: web::Path<i32>) -> impl Responder {
    let user_id = get_request_user(&req).await;
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        let _service = block(move || Service::find_by_id(*_id)).await?; 
        if _request_user.id == place.user_id {
            let form = service_form(payload.borrow_mut(), _request_user.id).await;
            Service.delete_service();
        }
    };
    HttpResponse::Ok()
}

