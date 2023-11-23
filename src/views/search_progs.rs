use actix_web::{
    HttpRequest,
    HttpResponse,
    web,
    web::block,
    error::InternalError,
    http::StatusCode,
};

use crate::utils::{
    establish_connection,
    get_request_user,
};

use sailfish::TemplateOnce;
use crate::models::User;


pub fn search_routes(config: &mut web::PlaceConfig) {
    config.route("/search_city/{q}/", web::get().to(search_city_page));
    config.route("/search_place/{q}/", web::get().to(search_place_page));
    config.route("/search_organization/{q}/", web::get().to(search_organization_page));
    config.route("/search_service/{q}/", web::get().to(search_service_page));
    config.route("/search_deceased/{q}/", web::get().to(search_deceased_page));
}

pub async fn search_page(req: HttpRequest, q: web::Path<String>) -> actix_web::Result<HttpResponse> {
    use crate::utils::get_device_and_ajax;

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let _q = q.clone();
    let _q_standalone = "%".to_owned() + &_q + "%";
    let template_types = get_template(&req);

    
        use crate::models::{Deceased, Citie, Place, Organization, Service, Deceased};

        let _connection = establish_connection();

        if get_request_user(&req).await.is_some() {
            let _request_user = get_request_user(&req).await.unwrap();
            let is_admin = _request_user.is_superuser();

            let deceased_list = Deceased::search_deceased(&_q_standalone, 3, 0, is_admin);
            let service_list = Deceased::search_place(&_q_standalone, 3, 0, is_admin);
            let service_list = Deceased::search_service(&_q_standalone, 3, 0, is_admin);
            let city_list = Deceased::search_city(&_q_standalone, 3, 0, is_admin);
            let organization_list = Deceased::search_organization(&_q_standalone, 3, 0, is_admin);

            let city_count = city_list.len();
            let service_count = service_list.len();
            let organization_count = organization_list.len();
            let service_count = service_list.len();
            let deceased_count = deceased_list.len();

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/search/all.stpl")]
                struct Template {
                    request_user:   User,
                    deceased_list:     Vec<Deceased>,
                    place_list:  Vec<Place>,
                    service_list:     Vec<Service>,
                    city_list:     Vec<Citie>,
                    organization_list:    Vec<Organization>,

                    deceased_count:    usize,
                    place_count: usize,
                    service_count:    usize,
                    city_count:    usize,
                    organization_count:   usize,
                    is_ajax:        i32,
                    q:              String,
                    template_types: i16,
                }
                let body = Template {
                    request_user:   _request_user,
                    deceased_list:     deceased_list,
                    place_list:  service_list,
                    service_list:     service_list,
                    city_list:     city_list,
                    organization_list:    organization_list,

                    deceased_count:    deceased_count,
                    place_count: service_count,
                    service_count:    service_count,
                    city_count:    city_count,
                    organization_count:   organization_count,
                    is_ajax:        is_ajax,
                    q:              _q,
                    template_types: template_types,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/search/all.stpl")]
                struct Template {
                    deceased_list:     Vec<Deceased>,
                    place_list:  Vec<Place>,
                    service_list:     Vec<Service>,
                    city_list:     Vec<Citie>,
                    organization_list:    Vec<Organization>,

                    deceased_count:    usize,
                    place_count: usize,
                    service_count:    usize,
                    city_count:    usize,
                    organization_count:   usize,
                    is_ajax:        i32,
                    q:              String,
                    template_types: i16,
                }
                let body = Template {
                    deceased_list:     deceased_list,
                    place_list:  service_list,
                    service_list:     service_list,
                    city_list:     city_list,
                    organization_list:    organization_list,

                    deceased_count:    deceased_count,
                    place_count: service_count,
                    service_count:    service_count,
                    city_count:    city_count,
                    organization_count:   organization_count,
                    is_ajax:        is_ajax,
                    q:              _q,
                    template_types: template_types,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
        else {
            let deceased_list = Deceased::search_deceased(&_q_standalone, 3, 0, false);
            let service_list = Deceased::search_place(&_q_standalone, 3, 0, false);
            let service_list = Deceased::search_service(&_q_standalone, 3, 0, false);
            let city_list = Deceased::search_city(&_q_standalone, 3, 0, false);
            let organization_list = Deceased::search_organization(&_q_standalone, 3, 0, false);

            let city_count = city_list.len();
            let service_count = service_list.len();
            let organization_count = organization_list.len();
            let service_count = service_list.len();
            let deceased_count = deceased_list.len();

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/search/anon_all.stpl")]
                struct Template {
                    deceased_list:     Vec<Deceased>,
                    place_list:  Vec<Place>,
                    service_list:     Vec<Service>,
                    city_list:     Vec<Citie>,
                    organization_list:    Vec<Organization>,

                    deceased_count:    usize,
                    place_count: usize,
                    service_count:    usize,
                    city_count:    usize,
                    organization_count:   usize,
                    is_ajax:        i32,
                    q:              String,
                    template_types: i16,
                }
                let body = Template {
                    deceased_list:     deceased_list,
                    place_list:  service_list,
                    service_list:     service_list,
                    city_list:     city_list,
                    organization_list:    organization_list,

                    deceased_count:    deceased_count,
                    place_count: service_count,
                    service_count:    service_count,
                    city_count:    city_count,
                    organization_count:   organization_count,
                    is_ajax:        is_ajax,
                    q:              _q,
                    template_types: template_types,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/search/anon_all.stpl")]
                struct Template {
                    deceased_list:     Vec<Deceased>,
                    place_list:  Vec<Place>,
                    service_list:     Vec<Service>,
                    city_list:     Vec<Citie>,
                    organization_list:    Vec<Organization>,

                    deceased_count:    usize,
                    place_count: usize,
                    service_count:    usize,
                    city_count:    usize,
                    organization_count:   usize,
                    is_ajax:        i32,
                    q:              String,
                    template_types: i16,
                }
                let body = Template {
                    deceased_list:     deceased_list,
                    place_list:  service_list,
                    service_list:     service_list,
                    city_list:     city_list,
                    organization_list:    organization_list,

                    deceased_count:    deceased_count,
                    place_count: service_count,
                    service_count:    service_count,
                    city_count:    city_count,
                    organization_count:   organization_count,
                    is_ajax:        is_ajax,
                    q:              _q,
                    template_types: template_types,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        
    }
}




pub async fn search_city_page(req: HttpRequest, q: web::Path<String>) -> actix_web::Result<HttpResponse> {
    use crate::utils::{get_device_and_ajax, get_page};

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let _q = q.clone();
    let template_types = get_template(&req);

        use crate::models::{City};

        let page = get_page(&req);
        let _connection = establish_connection();

        let _q_standalone = "%".to_owned() + &_q + "%";

        let mut next_page_number = 0;
        let offset: i32;
        let next_item: i32;
        if page > 1 {
            offset = (page - 1) * 20;
            next_item = page * 20 + 1;
        }
        else {
            offset = 0;
            next_item = 21;
        }

        if get_request_user(&req).await.is_some() {
            let _request_user = get_request_user(&req).await.unwrap();
            let is_admin = _request_user.is_superuser();
            let city_list = Citie::search_city(&_q_standalone, 20, offset.into(), is_admin);

            if Citie::search_city(&_q_standalone, 1, next_item.into(), is_admin).len() > 0 {
                next_page_number = page + 1;
            }

            let city_count = city_list.len();
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/search/city.stpl")]
                struct Template {
                    request_user:     User,
                    city_list:       Vec<Citie>,
                    city_count:      usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                    template_types:   i16,
                }
                let body = Template {
                    request_user:     _request_user,
                    city_list:       city_list,
                    city_count:      city_count,
                    is_ajax:          is_ajax,
                    q:                _q,
                    next_page_number: next_page_number,
                    template_types:   template_types,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/search/city.stpl")]
                struct Template {
                    city_list:       Vec<Citie>,
                    city_count:      usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                    template_types:   i16,
                }
                let body = Template {
                    city_list:       city_list,
                    city_count:      city_count,
                    is_ajax:          is_ajax,
                    q:                _q,
                    next_page_number: next_page_number,
                    template_types:   template_types,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
        else {
            let is_admin = false;
            let city_list = Citie::search_city(&_q_standalone, 20, offset.into(), is_admin);

            if Citie::search_city(&_q_standalone, 1, next_item.into(), is_admin).len() > 0 {
                next_page_number = page + 1;
            }

            let city_count = city_list.len();
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/search/anon_city.stpl")]
                struct Template {
                    city_list:       Vec<Citie>,
                    city_count:      usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                    template_types:   i16,
                }
                let body = Template {
                    city_list:       city_list,
                    city_count:      city_count,
                    is_ajax:          is_ajax,
                    q:                _q,
                    next_page_number: next_page_number,
                    template_types:   template_types,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/search/anon_city.stpl")]
                struct Template {
                    city_list:       Vec<Citie>,
                    city_count:      usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                    template_types:   i16,
                }
                let body = Template {
                    city_list:       city_list,
                    city_count:      city_count,
                    is_ajax:          is_ajax,
                    q:                _q,
                    next_page_number: next_page_number,
                    template_types:   template_types,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
    
}

pub async fn search_place_page(req: HttpRequest, q: web::Path<String>) -> actix_web::Result<HttpResponse> {
    use crate::utils::{get_device_and_ajax, get_page};

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let _q = q.clone();

        use crate::models::{Place};

        let page = get_page(&req);
        let _connection = establish_connection();

        let _q_standalone = "%".to_owned() + &_q + "%";

        let mut next_page_number = 0;
        let offset: i32;
        let next_item: i32;
        if page > 1 {
            offset = (page - 1) * 20;
            next_item = page * 20 + 1;
        }
        else {
            offset = 0;
            next_item = 21;
        }

        if get_request_user(&req).await.is_some() {
            let _request_user = get_request_user(&req).await.unwrap();
            let is_admin = _request_user.is_superuser();
            let place_list = Place::search_place(&_q_standalone, 20, offset.into(), is_admin);

            if Place::search_place(&_q_standalone, 1, next_item.into(), is_admin).len() > 0 {
                next_page_number = page + 1;
            }
            let place_count = place_list.len();

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/search/place.stpl")]
                struct Template {
                    request_user:     User,
                    place_list:    Vec<Place>,
                    place_count:   usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                    template_types:   i16,
                }
                let body = Template {
                    request_user:     _request_user,
                    place_list:    place_list,
                    place_count:   place_count,
                    is_ajax:          is_ajax,
                    q:                _q,
                    next_page_number: next_page_number,
                    template_types:   template_types,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/search/place.stpl")]
                struct Template {
                    place_list:    Vec<Place>,
                    place_count:   usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                    template_types:   i16,
                }
                let body = Template {
                    place_list:    place_list,
                    place_count:   place_count,
                    is_ajax:          is_ajax,
                    q:                _q,
                    next_page_number: next_page_number,
                    template_types:   template_types,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
        else {
            let is_admin = false;
            let place_list = Place::search_place(&_q_standalone, 20, offset.into(), is_admin);

            if Place::search_place(&_q_standalone, 1, next_item.into(), is_admin).len() > 0 {
                next_page_number = page + 1;
            }
            let place_count = place_list.len();

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/search/anon_place.stpl")]
                struct Template {
                    place_list:    Vec<Place>,
                    place_count:   usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                    template_types:   i16,
                }
                let body = Template {
                    place_list:    place_list,
                    place_count:   place_count,
                    is_ajax:          is_ajax,
                    q:                _q,
                    next_page_number: next_page_number,
                    template_types:   template_types,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/search/anon_place.stpl")]
                struct Template {
                    place_list:    Vec<Place>,
                    place_count:   usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                    template_types:   i16,
                }
                let body = Template {
                    place_list:    place_list,
                    place_count:   place_count,
                    is_ajax:          is_ajax,
                    q:                _q,
                    next_page_number: next_page_number,
                    template_types:   template_types,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
    
}

pub async fn search_organization_page(req: HttpRequest, q: web::Path<String>) -> actix_web::Result<HttpResponse> {
    use crate::utils::{get_device_and_ajax, get_page};

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let _q = q.clone();

        use crate::models::{Organization};

        let page = get_page(&req);

        let _connection = establish_connection();
        let _q_standalone = "%".to_owned() + &_q + "%";

        let mut next_page_number = 0;
        let offset: i32;
        let next_item: i32;
        if page > 1 {
            offset = (page - 1) * 20;
            next_item = page * 20 + 1;
        }
        else {
            offset = 0;
            next_item = 21;
        }

        if get_request_user(&req).await.is_some() {
            let _request_user = get_request_user(&req).await.unwrap();
            let is_admin = _request_user.is_superuser();
            let organization_list = Organization::search_organization(&_q_standalone, 20, offset.into(), is_admin);

            if Organization::search_organization(&_q_standalone, 1, next_item.into(), is_admin).len() > 0 {
                next_page_number = page + 1;
            }

            let organization_count = organization_list.len();

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/search/organization.stpl")]
                struct Template {
                    request_user:     User,
                    organization_list:      Vec<Organization>,
                    organization_count:     usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                    template_types:   i16,
                }
                let body = Template {
                    request_user:     _request_user,
                    organization_list:       organization_list,
                    organization_count:      organization_count,
                    is_ajax:          is_ajax,
                    q:                _q,
                    next_page_number: next_page_number,
                    template_types:   template_types,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/search/organization.stpl")]
                struct Template {
                    organization_list:      Vec<Organization>,
                    organization_count:     usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                    template_types:   i16,
                }
                let body = Template {
                    organization_list:      organization_list,
                    organization_count:     organization_count,
                    is_ajax:          is_ajax,
                    q:                _q,
                    next_page_number: next_page_number,
                    template_types:   template_types,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
        else {
            let is_admin = false;
            let organization_list = Organization::search_organization(&_q_standalone, 20, offset.into(), is_admin);

            if Organization::search_organization(&_q_standalone, 1, next_item.into(), is_admin).len() > 0 {
                next_page_number = page + 1;
            }

            let organization_count = organization_list.len();

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/search/anon_organization.stpl")]
                struct Template {
                    organization_list:      Vec<Organization>,
                    organization_count:     usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                    template_types:   i16,
                }
                let body = Template {
                    organization_list:      organization_list,
                    organization_count:     organization_count,
                    is_ajax:          is_ajax,
                    q:                _q,
                    next_page_number: next_page_number,
                    template_types:   template_types,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/search/anon_organization.stpl")]
                struct Template {
                    organization_list:      Vec<Organization>,
                    organization_count:     usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                    template_types:   i16,
                }
                let body = Template {
                    organization_list:      organization_list,
                    organization_count:     organization_count,
                    is_ajax:          is_ajax,
                    q:                _q,
                    next_page_number: next_page_number,
                    template_types:   template_types,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
    
}

pub async fn search_service_page(req: HttpRequest, q: web::Path<String>) -> actix_web::Result<HttpResponse> {
    use crate::utils::{get_device_and_ajax, get_page};

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let _q = q.clone();

        use crate::models::{Service};

        let page = get_page(&req);
        let _connection = establish_connection();
        let _q_standalone = "%".to_owned() + &_q + "%";

        let mut next_page_number = 0;
        let offset: i32;
        let next_item: i32;
        if page > 1 {
            offset = (page - 1) * 20;
            next_item = page * 20 + 1;
        }
        else {
            offset = 0;
            next_item = 21;
        }

        if get_request_user(&req).await.is_some() {
            let _request_user = get_request_user(&req).await.unwrap();
            let is_admin = _request_user.is_superuser();
            let service_list = Service::search_service(&_q_standalone, 20, offset.into(), is_admin);

            if Service::search_service(&_q_standalone, 1, next_item.into(), is_admin).len() > 0 {
                next_page_number = page + 1;
            }

            let service_count = service_list.len();

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/search/service.stpl")]
                struct Template {
                    request_user:     User,
                    service_list:       Vec<Service>,
                    service_count:      usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                    template_types:   i16,
                }
                let body = Template {
                    request_user:     _request_user,
                    service_list:       service_list,
                    service_count:      service_count,
                    is_ajax:          is_ajax,
                    q:                _q,
                    next_page_number: next_page_number,
                    template_types:   template_types,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/search/service.stpl")]
                struct Template {
                    service_list:       Vec<Service>,
                    service_count:      usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                    template_types:   i16,
                }
                let body = Template {
                    service_list:       service_list,
                    service_count:      service_count,
                    is_ajax:          is_ajax,
                    q:                _q,
                    next_page_number: next_page_number,
                    template_types:   template_types,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
        else {
            let is_admin = false;
            let service_list = Service::search_service(&_q_standalone, 20, offset.into(), is_admin);

            if Service::search_service(&_q_standalone, 1, next_item.into(), is_admin).len() > 0 {
                next_page_number = page + 1;
            }
            let service_count = service_list.len();

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/search/anon_service.stpl")]
                struct Template {
                    service_list:       Vec<Service>,
                    service_count:      usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                    template_types:   i16,
                }
                let body = Template {
                    service_list:       service_list,
                    service_count:      service_count,
                    is_ajax:          is_ajax,
                    q:                _q,
                    next_page_number: next_page_number,
                    template_types:   template_types,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/search/anon_service.stpl")]
                struct Template {
                    service_list:       Vec<Service>,
                    service_count:      usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                    template_types:   i16,
                }
                let body = Template {
                    service_list:       service_list,
                    service_count:      service_count,
                    is_ajax:          is_ajax,
                    q:                _q,
                    next_page_number: next_page_number,
                    template_types:   template_types,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
    
}

pub async fn search_deceased_page(req: HttpRequest, q: web::Path<String>) -> actix_web::Result<HttpResponse> {
    use crate::utils::{get_device_and_ajax, get_page};

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let _q = q.clone();

        use crate::models::{Deceased};

        let page = get_page(&req);
        let _connection = establish_connection();
        let _q_standalone = "%".to_owned() + &_q + "%";

        let mut next_page_number = 0;
        let offset: i32;
        let next_item: i32;
        if page > 1 {
            offset = (page - 1) * 20;
            next_item = page * 20 + 1;
        }
        else {
            offset = 0;
            next_item = 21;
        }

        if get_request_user(&req).await.is_some() {
            let _request_user = get_request_user(&req).await.unwrap();

            let is_admin = _request_user.is_superuser();
            let deceased_list = Deceased::search_deceased(&_q_standalone, 20, offset.into(), is_admin);

            if Deceased::search_deceased(&_q_standalone, 1, next_item.into(), is_admin).len() > 0 {
                next_page_number = page + 1;
            }

            let deceased_count = deceased_list.len();

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/search/deceased.stpl")]
                struct Template {
                    request_user:     User,
                    deceased_list:       Vec<Deceased>,
                    deceased_count:      usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                    template_types:   i16,
                }
                let body = Template {
                    request_user:     _request_user,
                    deceased_list:       deceased_list,
                    deceased_count:      deceased_count,
                    is_ajax:          is_ajax,
                    q:                _q,
                    next_page_number: next_page_number,
                    template_types:   template_types,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/search/deceased.stpl")]
                struct Template {
                    deceased_list:       Vec<Deceased>,
                    deceased_count:      usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                    template_types:   i16,
                }
                let body = Template {
                    deceased_list:       deceased_list,
                    deceased_count:      deceased_count,
                    is_ajax:          is_ajax,
                    q:                _q,
                    next_page_number: next_page_number,
                    template_types:   template_types,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
        else {
            let is_admin = false;
            let deceased_list = Deceased::search_deceased(&_q_standalone, 20, offset.into(), is_admin);

            if Deceased::search_deceased(&_q_standalone, 1, next_item.into(), is_admin).len() > 0 {
                next_page_number = page + 1;
            }
            let deceased_count = deceased_list.len();

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/search/anon_deceased.stpl")]
                struct Template {
                    deceased_list:       Vec<Deceased>,
                    deceased_count:      usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                    template_types:   i16,
                }
                let body = Template {
                    deceased_list:       deceased_list,
                    deceased_count:      deceased_count,
                    is_ajax:          is_ajax,
                    q:                _q,
                    next_page_number: next_page_number,
                    template_types:   template_types,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/search/anon_deceased.stpl")]
                struct Template {
                    deceased_list:       Vec<Deceased>,
                    deceased_count:      usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                    template_types:   i16,
                }
                let body = Template {
                    deceased_list:       deceased_list,
                    deceased_count:      deceased_count,
                    is_ajax:          is_ajax,
                    q:                _q,
                    next_page_number: next_page_number,
                    template_types:   template_types,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
    
}


