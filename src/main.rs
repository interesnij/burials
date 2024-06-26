#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;

//use actix::Actor;
use actix_cors::Cors;
use dotenv::dotenv;
use env_logger;

pub mod schema;
pub mod routes;
mod errors;

use actix_web::{
    HttpServer,
    App,
    middleware::{
        Compress, 
        Logger, 
    },
    web,
    http,
};

use actix_files::Files;
use crate::routes::routes;
//use std::cell::Cell;
//use std::sync::atomic::{AtomicUsize, Ordering};
//use std::sync::{Arc, Mutex};

#[macro_use]
mod utils;
#[macro_use]
mod views;
#[macro_use]
mod models;

//use crate::utils::AppState;
use crate::views::not_found;

//static SERVER_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));
    //let server = websocket::Server::new().start();

    HttpServer::new(move || {
        let _files = Files::new("/static", "static/").show_files_listing();
        let _files2 = Files::new("/media", "media/").show_files_listing();
        //let messages = Arc::new(Mutex::new(vec![]));

        App::new()  
            //.data(AppState {
            //    server_id: SERVER_COUNTER.fetch_add(1, Ordering::SeqCst),
            //    request_count: Cell::new(0),
            //    messages: messages.clone(),
            //})
            //.wrap(Logger::default())
            .wrap(Compress::default())
            //.data(server.clone())
            .service(_files)
            .service(_files2)
            .configure(routes)
            .default_service(web::route().to(not_found))
    })
    .bind("89.108.82.212:9099")?
    .run()
    .await
}
