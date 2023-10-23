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




pub fn page_routes(config: &mut web::ServiceConfig) {
    config.route("/", web::get().to(index_page));
}


pub async fn index_page (
    req: HttpRequest,
    session: Session,
    websocket_srv: Data<Addr<Server>>) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let template_types = get_template(&req);

    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Главная страница".to_string(),
            "вебсервисы - Комплексное, экспертное создание и развитие высоконагруженных веб-ресурсов".to_string(),
            "/".to_string(),
            "/static/images/dark/store.jpg".to_string(),
            template_types,
        ).await
    }
    else {
        use crate::schema::stat_pages::dsl::stat_pages;
        use crate::models::{Blog, Service, Store, Wiki, Work};
        use crate::websocket::MessageToClient;

        let _connection = establish_connection();
        let _stat: StatPage;

        let _stats = stat_pages
            .filter(schema::stat_pages::types.eq(1))
            .first::<StatPage>(&_connection);
        if _stats.is_ok() {
            _stat = _stats.expect("E");
            diesel::update(&_stat)
                .set(schema::stat_pages::now_u.eq(_stat.now_u + 1))
                .get_result::<StatPage>(&_connection)
                .expect("Error.");
        }
        else {
            use crate::models::NewStatPage;
            let form = NewStatPage {
                types:   1,
                view:    0,
                height:  0.0,
                seconds: 0,
                now_u:   1,
            };
            _stat = diesel::insert_into(schema::stat_pages::table)
                .values(&form)
                .get_result::<StatPage>(&_connection)
                .expect("Error.");

        }
        //if let Ok(res) = to_value(_stat.now_u.to_string()) {
        //    let msg = MessageToClient::new("page_view", _stat.types.into(), res);
        //    websocket_srv.do_send(msg);
        //}

        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);
            let is_admin = _request_user.is_superuser();
            //User::create_superuser(_request_user.id);
            let _last_works = Item::get_works(3, 0, is_admin);
            let _last_services = Item::get_services(3, 0, is_admin);
            let _last_wikis = Item::get_wikis(3, 0, is_admin);
            let _last_blogs = Item::get_blogs(3, 0, is_admin);
            let _last_stores = Item::get_stores(3, 0, is_admin);

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/main/mainpage.stpl")]
                struct Template {
                    request_user:   User,
                    last_works:     Vec<Work>,
                    last_services:  Vec<Service>,
                    last_wikis:     Vec<Wiki>,
                    last_blogs:     Vec<Blog>,
                    last_stores:    Vec<Store>,
                    is_ajax:        i32,
                    stat:           StatPage,
                    template_types: i16,
                }
                let body = Template {
                    request_user:   _request_user,
                    last_works:     _last_works,
                    last_services:  _last_services,
                    last_wikis:     _last_wikis,
                    last_blogs:     _last_blogs,
                    last_stores:    _last_stores,
                    is_ajax:        is_ajax,
                    stat:           _stat,
                    template_types: template_types,
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
                    last_works:     Vec<Work>,
                    last_services:  Vec<Service>,
                    last_wikis:     Vec<Wiki>,
                    last_blogs:     Vec<Blog>,
                    last_stores:    Vec<Store>,
                    is_ajax:        i32,
                    stat:           StatPage,
                    template_types: i16,
                }
                let body = Template {
                    request_user:   _request_user,
                    last_works:     _last_works,
                    last_services:  _last_services,
                    last_wikis:     _last_wikis,
                    last_blogs:     _last_blogs,
                    last_stores:    _last_stores,
                    is_ajax:        is_ajax,
                    stat:           _stat,
                    template_types: template_types,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
        else {
            let _last_works = Item::get_works(3, 0, false);
            let _last_services = Item::get_services(3, 0, false);
            let _last_wikis = Item::get_wikis(3, 0, false);
            let _last_blogs = Item::get_blogs(3, 0, false);
            let _last_stores = Item::get_stores(3, 0, false);

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/main/anon_mainpage.stpl")]
                struct Template {
                    last_works:     Vec<Work>,
                    last_services:  Vec<Service>,
                    last_wikis:     Vec<Wiki>,
                    last_blogs:     Vec<Blog>,
                    last_stores:    Vec<Store>,
                    is_ajax:        i32,
                    stat:           StatPage,
                    template_types: i16,
                }
                let body = Template {
                    last_works:     _last_works,
                    last_services:  _last_services,
                    last_wikis:     _last_wikis,
                    last_blogs:     _last_blogs,
                    last_stores:    _last_stores,
                    is_ajax:        is_ajax,
                    stat:           _stat,
                    template_types: template_types,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/main/anon_mainpage.stpl")]
                struct Template {
                    last_works:     Vec<Work>,
                    last_services:  Vec<Service>,
                    last_wikis:     Vec<Wiki>,
                    last_blogs:     Vec<Blog>,
                    last_stores:    Vec<Store>,
                    is_ajax:        i32,
                    stat:           StatPage,
                    template_types: i16,
                }
                let body = Template {
                    last_works:     _last_works,
                    last_services:  _last_services,
                    last_wikis:     _last_wikis,
                    last_blogs:     _last_blogs,
                    last_stores:    _last_stores,
                    is_ajax:        is_ajax,
                    stat:           _stat,
                    template_types: template_types,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
    }
}










