// Подключение необходимых модулей
use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use crate::schema::*;

// Структура для представления данных об организации
#[derive(Queryable, Serialize, Deserialize)]
pub struct Organization {
    pub id: i32,                     // Уникальный идентификатор организации
    pub name: String,                // Название организации
    pub description: String,         // Описание организации
    pub director: String,            // Руководитель организации
    pub phone_number: String,        // Номер телефона организации
    pub place_id: i32,               // Идентификатор места организации
    pub city_id: i32,                // Идентификатор города организации
    pub region_id: i32,              // Идентификатор региона организации
    pub country_id: i32,             // Идентификатор страны организации
    pub service_ids: Vec<i32>,       // Идентификаторы предоставляемых услуг
    pub working_hours: String,       // Время работы организации
    pub website: Option<String>,     // Веб-сайт организации (может быть пустым)
    pub photo_link: Option<String>,  // Ссылка на фотографию организации (может быть пустой)
    pub messenger_links: Vec<String>, // Ссылки на мессенджеры организации
    pub location_coordinates: Point, // Координаты расположения организации
}

// Структура для создания новой организации
#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "organizations"]
pub struct NewOrganization {
    pub name: String,
    pub description: String,
    pub director: String,
    pub phone_number: String,
    pub place_id: i32,
    pub city_id: i32,
    pub region_id: i32,
    pub country_id: i32,
    pub service_ids: Vec<i32>,
    pub working_hours: String,
    pub website: Option<String>,
    pub photo_link: Option<String>,
    pub messenger_links: Vec<String>,
    pub location_coordinates: Point,
}
