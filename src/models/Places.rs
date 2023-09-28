use diesel::prelude::*;
use super::schema::*;


// Структура для таблицы Places
#[derive(Debug, Queryable, Identifiable, Associations)]
pub struct Place {
    pub id: i32,                             // Уникальный идентификатор места
    pub city_id: i32,                        // Идентификатор города, в котором находится место
    pub region_id: i32,                      // Идентификатор региона, к которому принадлежит место
    pub country_id: i32,                     // Идентификатор страны, к которой принадлежит место
    pub cemetery_name: String,               // Название кладбища (места)
    pub description: Option<String>,         // Описание кладбища
    pub summer_hours: Option<String>,        // Часы работы в летнее время
    pub winter_hours: Option<String>,        // Часы работы в зимнее время
    pub photo_links: Option<String>,          // Ссылки на фотографии кладбища
    pub address: Option<String>,             // Адрес кладбища
    pub deceased_id: i32,                    // Идентификатор покойников
    pub organization_id: i32,                // Идентификатор организаций
    pub burial_count: i32,                   // Количество захоронений
    pub cemetery_director: Option<String>,    // Руководитель кладбища
    pub cemetery_phone_number: Option<String>,// Номер телефона кладбища
}

// Структура для создания новой записи Place
#[derive(Debug, Insertable)]
#[table_name = "places"]
pub struct NewPlace {
    pub city_id: i32,                        // Идентификатор города, в котором находится место
    pub region_id: i32,                      // Идентификатор региона, к которому принадлежит место
    pub country_id: i32,                     // Идентификатор страны, к которой принадлежит место
    pub cemetery_name: String,               // Название кладбища (места)
    pub description: Option<String>,         // Описание кладбища
    pub summer_hours: Option<String>,        // Часы работы в летнее время
    pub winter_hours: Option<String>,        // Часы работы в зимнее время
    pub photo_links: Option<String>,         // Ссылки на фотографии кладбища
    pub address: Option<String>,             // Адрес кладбища
    pub deceased_id: i32,                    // Идентификатор покойников
    pub organization_id: i32,                // Идентификатор организаций
    pub burial_count: i32,                   // Количество захоронений
    pub cemetery_director: Option<String>,    // Руководитель кладбища
    pub cemetery_phone_number: Option<String>,// Номер телефона кладбища
}
