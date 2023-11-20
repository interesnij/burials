use actix_web::{
    HttpRequest,
    HttpResponse, 
    Responder,
    web,
    error::InternalError,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use crate::utils::{
    establish_connection,
    verify,
    get_request_user,
    gen_jwt,
};
use crate::diesel::{
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
};
use crate::schema;
use futures::StreamExt;
use crate::models::{User, NewUser};
use crate::errors::AuthError;
use actix_multipart::{Field, Multipart};
use sailfish::TemplateOnce;
use std::borrow::BorrowMut;


pub fn auth_routes(config: &mut web::ServiceConfig) {
    config.service(web::resource("/login/")
        .route(web::get().to(login_page))
        .route(web::post().to(login))
    );
    config.service(web::resource("/signup/")
        .route(web::get().to(signup_page))
        .route(web::post().to(process_signup))
    );
    config.route("/logout/", web::get().to(logout_page));
}


pub async fn signup_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    if get_request_user.await.is_some(&req) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/auth/signup.stpl")]
            struct Template {
                is_ajax: i32,
            }
            let body = Template {
                is_ajax: is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/auth/signup.stpl")]
            struct Template {
                is_ajax: i32,
            }
            let body = Template {
                is_ajax: is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}
pub async fn login_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    if get_request_user.await.is_some(&req) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/auth/login.stpl")]
            struct Template {
                is_ajax: i32,
            }
            let body = Template {
                is_ajax: is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/auth/login.stpl")]
            struct Template {
                is_ajax: i32,
                }
            let body = Template {
                is_ajax: is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn logout_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    if get_request_user(&req).await.is_none() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        crate::utils::remove_token(&req);
        crate::viwes::index_page(req).await
    }
}

fn find_user(data: LoginUser2) -> Result<User, AuthError> {
    let _connection = establish_connection();
    let item = schema::users::table
        .filter(schema::users::username.eq(&data.username))
        .first::<User>(&_connection)
        .expect("Error.");

    if let Ok(matching) = verify(&item.password, &data.password) {
        if matching {
            return Ok(item);
        }
    }
    
    Err(AuthError::NotFound(String::from("User not found")))
}

fn handle_sign_in(data: LoginUser2, req: &HttpRequest) -> i16 {
    let _connection = establish_connection();
    let result = find_user(data);

    match result {  
        Ok(_user) => { 
            if verify(data.password.as_str(), _user.password.as_str()).unwrap() {
                let token = gen_jwt(_user.id, &"MY_SECRET".to_string());
            } 
            match token {
                Ok(token_str) => {
                    crate::utils::set_token(&token_str, &_user.id.to_string());
                    1
                },
                Err(err) => 0,
            }
        },
        Err(err) => 0,
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LoginUser2 {
    pub username: String,
    pub password: String,
}
pub async fn login_form(payload: &mut Multipart) -> LoginUser2 {
    let mut form: LoginUser2 = LoginUser2 {
        username: "".to_string(),
        password: "".to_string(),
    };

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");
        while let Some(chunk) = field.next().await {
            let data = chunk.expect("split_payload err chunk");
            if let Ok(s) = std::str::from_utf8(&data) {
                let data_string = s.to_string();
                if field.name() == "username" {
                    form.username = data_string
                } else if field.name() == "password" {
                    form.password = data_string
                }
            }
        }
    }
    form
}

pub async fn login(mut payload: Multipart, req: HttpRequest) -> impl Responder {
    if get_request_user(&req).await.is_some() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let form = login_form(payload.borrow_mut()).await;
        println!("{:?}", form.username.clone());
        println!("{:?}", form.password.clone());
        handle_sign_in(form, &req);
        crate::viwes::index_page(req).await
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NewUserForm {
    pub username: String,
    pub email:    String,
    pub password: String,
}
pub async fn signup_form(payload: &mut Multipart) -> NewUserForm {
    let mut form: NewUserForm = NewUserForm {
        username: "".to_string(),
        email:    "".to_string(),
        password: "".to_string(),
    };

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");
        while let Some(chunk) = field.next().await {
            let data = chunk.expect("split_payload err chunk");
            if let Ok(s) = std::str::from_utf8(&data) {
                let data_string = s.to_string();
                if field.name() == "username" {
                    form.username = data_string
                }
                else if field.name() == "email" {
                    form.email = data_string
                }
                else if field.name() == "password" {
                    form.password = data_string
                }
            }
        }
    }
    form
}
pub async fn process_signup(req: HttpRequest, mut payload: Multipart) -> impl Responder {
    use crate::utils::hash_password; 

    // Если пользователь не аноним, то отправляем его на страницу новостей
    if get_request_user(&req).await.is_some() {
        HttpResponse::Ok().content_type("text/html; charset=utf-8").body("")
    }
    else { 
        let form = signup_form(payload.borrow_mut()).await;
        let _connection = establish_connection();
        let _password = hash(form.password, 8).unwrap();
        let form_user = NewUser {
            username: form.username.clone(),
            email:    form.email.clone(),
            password: _password.clone(),
            bio:      None,
            image:    None,
            perm:     1,
        }; 
        println!("{:?}", form.username.clone());
        println!("{:?}", form.email.clone());
        println!("{:?}", form.password.clone());

        let _new_user = diesel::insert_into(schema::users::table)
            .values(&form_user)
            .get_result::<User>(&_connection)
            .expect("Error saving user.");

        //set_current_user(&_user);
        crate::viwes::index_page(req).await
    }
}
