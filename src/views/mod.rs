pub mod auth;
pub mod deceaseds;
//pub mod organizations;
pub mod page;
pub mod places;
//pub mod progs;
//pub mod services;
pub mod users;
pub mod admin;


pub use self::{
    auth::*,
    deceaseds::*,
    //organizations::*,
    page::*,
    places::*,
    //progs::*,
    //services::*,
    users::*,
    admin::*,
};
