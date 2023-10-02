// Подключаем необходимые модули (use)
use diesel::prelude::*;
use crate::schema::countries; // Замените на фактический путь к модулю schema

// Структура Countries
#[derive(Queryable, Debug)] // Добавляем Queryable для работы с базой данных и Debug для отладки
pub struct Countries {
    pub id: i32, // Уникальный идентификатор
    pub name: String, // Название страны
    pub geo_id: i32, // Идентификатор географического местоположения
}

// Структура NewCountries для создания новых объектов
#[derive(Insertable, Debug)]
#[table_name = "countries"]
pub struct NewCountries {
    pub name: String, // Название страны
    pub geo_id: i32, // Идентификатор географического местоположения
}


use crate::schema::regions; // Замените на фактический путь к модулю schema

// Структура Regions
#[derive(Queryable, Debug)] // Добавляем Queryable для работы с базой данных и Debug для отладки
pub struct Regions {
    pub id: i32, // Уникальный идентификатор
    pub name: String, // Название региона
    pub geo_id: i32, // Идентификатор географического местоположения
    pub country_id: i32, // Идентификатор страны, к которой принадлежит регион
}

// Структура NewRegions для создания новых объектов
#[derive(Insertable, Debug)]
#[table_name = "regions"]
pub struct NewRegions {
    pub name: String, // Название региона
    pub geo_id: i32, // Идентификатор географического местоположения
    pub country_id: i32, // Идентификатор страны, к которой принадлежит регион
}



use crate::schema::citys; // Замените на фактический путь к модулю schema

// Структура Citys
#[derive(Queryable, Debug)] // Добавляем Queryable для работы с базой данных и Debug для отладки
pub struct Citys {
    pub id: i32, // Уникальный идентификатор
    pub name: String, // Название города
    pub geo_id: i32, // Идентификатор географического местоположения
    pub region_id: i32, // Идентификатор региона, к которому принадлежит город
    pub country_id: i32, // Идентификатор страны, к которой принадлежит город
    pub lat: f64, // Широта местоположения
    pub lon: f64, // Долгота местоположения
}

// Структура NewCitys для создания новых объектов
#[derive(Insertable, Debug)]
#[table_name = "citys"]
pub struct NewCitys {
    pub name: String, // Название города
    pub geo_id: i32, // Идентификатор географического местоположения
    pub region_id: i32, // Идентификатор региона, к которому принадлежит город
    pub country_id: i32, // Идентификатор страны, к которой принадлежит город
    pub lat: f64, // Широта местоположения
    pub lon: f64, // Долгота местоположения
}
