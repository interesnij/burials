


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
    Deceased,
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

pub fn place_routes(config: &mut web::ServiceConfig) {

    config.route("/all_place_city/{id}/", web::get().to(all_place_city_page));
    config.route("/all_place_region/{id}/", web::get().to(all_place_region_page));
    config.route("/all_place_countries/{id}/", web::get().to(all_place_countries_page));

    config.route("/place/{id}/", web::get().to(place_page));

    config.route("/delete_place/{id}/", web::post().to(delete_place_page));
    config.route("/edit_place/{id}/", web::post().to(edit_place_page));
    config.route("/create_place/", web::post().to(create_place_page));

}


// pub fn deceased_routes(config: &mut web::ServiceConfig) {
//     config.route("/all_deceased_place/{id}/", web::get().to(all_deceased_place_page));
    // config.route("/deceased/{id}/", web::get().to(deceased_page));
    // config.route("/delete_deceased/{id}/", web::post().to(delete_deceased_page));
    // config.route("/edit_deceased/{id}/", web::post().to(edit_deceased_page));
    // config.route("/create_deceased/", web::post().to(create_deceased_page));
// }

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
//Получение всех кладбищ одного города

pub async fn all_place_city_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let is_desctop = is_desctop(&req);
    let _city = block(move || City::find_by_id(*_id)).await?;
    let _place = block(move || Deceased::get_all_place()).await?;
    let user_id = get_request_user_id(&req);
    if user_id.is_some() {
        let _request_user = get_user(user_id.unwrap());
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/place/all_place_city_page.stpl")]
            struct Template {
                request_user:   User,
                city:          Vec<City>,
                all_places:  Vec<Place>,
            }
            let body = Template {
                request_user:   _request_user,
                city:           _city,
                all_places:   _places,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/place/all_place_city_page.stpl")]
            struct Template {
                request_user:   User,
                city:          Vec<City>,
                all_places:  Vec<Place>,
            }
            let body = Template {
                request_user:   _request_user,
                city:           _city,
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
            #[template(path = "desctop/place/anon_all_place_city_page.stpl")]
            struct Template {
                request_user:   User,
                city:          Vec<City>,
                all_places:  Vec<Place>,
            }
            let body = Template {
                request_user:   _request_user,
                city:           _city,
                all_places:   _places,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/place/anon_all_place_city_page.stpl")]
            struct Template {
                request_user:   User,
                city:           Vec<City>,
                all_places:       Vec<Place>,
            }
            let body = Template {
                request_user:   _request_user,
                city:          _city,
                all_places:  _places,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}
//Получение всех кладбищ одного региона

pub async fn all_place_region_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let is_desctop = is_desctop(&req);
    let _region = block(move || Region::find_by_id(*_id)).await?;
    let _place = block(move || Deceased::get_all_place()).await?;
    let user_id = get_request_user_id(&req);
    if user_id.is_some() {
        let _request_user = get_user(user_id.unwrap());
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/place/all_place_region_page.stpl")]
            struct Template {
                request_user:   User,
                region:          Vec<Region>,
                all_places:  Vec<Place>,
            }
            let body = Template {
                request_user:   _request_user,
                region:           _region,
                all_places:   _places,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/place/all_place_region_page.stpl")]
            struct Template {
                request_user:   User,
                region:          Vec<Region>,
                all_places:  Vec<Place>,
            }
            let body = Template {
                request_user:   _request_user,
                region:           _region,
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
            #[template(path = "desctop/place/anon_all_place_region_page.stpl")]
            struct Template {
                request_user:   User,
                region:          Vec<Region>,
                all_places:  Vec<Place>,
            }
            let body = Template {
                request_user:   _request_user,
                region:           _region,
                all_places:   _places,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/place/anon_all_place_region_page.stpl")]
            struct Template {
                request_user:   User,
                region:           Vec<Region>,
                all_places:       Vec<Place>,
            }
            let body = Template {
                request_user:   _request_user,
                region:          _region,
                all_places:  _places,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}
//Получение всех кладбищ одной страны

pub async fn all_place_countries_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let is_desctop = is_desctop(&req);
    let _countries = block(move || Countries::find_by_id(*_id)).await?;
    let _place = block(move || Deceased::get_all_place()).await?;
    let user_id = get_request_user_id(&req);
    if user_id.is_some() {
        let _request_user = get_user(user_id.unwrap());
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/place/all_place_countries_page.stpl")]
            struct Template {
                request_user:   User,
                countries:          Vec<Countries>,
                all_places:  Vec<Place>,
            }
            let body = Template {
                request_user:   _request_user,
                countries:           _countries,
                all_places:   _places,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/place/all_place_countries_page.stpl")]
            struct Template {
                request_user:   User,
                countries:          Vec<Countries>,
                all_places:  Vec<Place>,
            }
            let body = Template {
                request_user:   _request_user,
                countries:           _countries,
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
            #[template(path = "desctop/place/anon_all_place_countries_page.stpl")]
            struct Template {
                request_user:   User,
                countries:          Vec<Countries>,
                all_places:  Vec<Place>,
            }
            let body = Template {
                request_user:   _request_user,
                countries:           _countries,
                all_places:   _places,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/place/anon_all_place_countries_page.stpl")]
            struct Template {
                request_user:   User,
                countries:           Vec<Countries>,
                all_places:       Vec<Place>,
            }
            let body = Template {
                request_user:   _request_user,
                countries:          _countries,
                all_places:  _places,
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
    let is_desctop = is_desctop(&req);
    let _place = block(move || Place::find_by_id(*_id)).await?;
    let user_id = get_request_user_id(&req);
    if user_id.is_some() {
        let _request_user = get_user(user_id.unwrap());
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/place/place_page.stpl")]
            struct Template {
                request_user:   User,
                place:       Place,
            }
            let body = Template {
                request_user:   _request_user,
                place:      _place,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/place/place_page.stpl")]
            struct Template {
                request_user:   User,
                place:       Place,
            }
            let body = Template {
                request_user:   _request_user,
                place:       _place,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
    else {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/place/anon_place_page.stpl")]
            struct Template {
                request_user:   User,
                place:       Place,
            }
            let body = Template {
                request_user:   _request_user,
                place:       _place,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/place/anon_place_page.stpl")]
            struct Template {
                request_user:   User,
                place:       Place,
            }
            let body = Template {
                request_user:   _request_user,
                place:       _place,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn create_place_page(req: HttpRequest, mut payload: Multipart) -> impl Responder {
    let user_id = get_request_user_id(&req);
    if user_id.is_some() {
        let _request_user = get_user(user_id.unwrap());
        if _request_user.is_admin() {
            let form = place_form(payload.borrow_mut(), _request_user.id).await;
            Place::create_place(form);
        }
    };
    HttpResponse::Ok()
}



pub async fn edit_place_page(req: HttpRequest, _id: web::Path<i32>) -> impl Responder {
    let user_id = get_request_user_id(&req);
    if user_id.is_some() {
        let _request_user = get_user(user_id.unwrap());
        let _place = block(move || Place::find_by_id(*_id)).await?; 
        if _request_user.id == place.user_id {
            let form = place_form(payload.borrow_mut(), _request_user.id).await;
            Place.edit_place(form);
        }
    };
    HttpResponse::Ok()
}
pub async fn delete_place_page(req: HttpRequest, _id: web::Path<i32>) -> impl Responder {
    let user_id = get_request_user_id(&req);
    if user_id.is_some() {
        let _request_user = get_user(user_id.unwrap());
        let _place = block(move || Place::find_by_id(*_id)).await?; 
        if _request_user.id == place.user_id {
            let form = place_form(payload.borrow_mut(), _request_user.id).await;
            Place.delete_place();
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

