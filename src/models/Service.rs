// Подключаем необходимые модули
use diesel::sql_types::Int4;
use diesel::sql_types::Text;
use diesel::sql_types::Numeric;
use diesel::sql_types::Array;
use crate::schema::services; // Подставьте свой путь к схеме

// Структура Service представляет услугу
#[derive(Queryable)]
pub struct Service {
    pub id: i32,                // Уникальный идентификатор услуги
    pub organization_id: i32,   // Идентификатор организации, предоставляющей услугу
    pub description: String,    // Описание услуги
    pub photo_link: String,     // Ссылка на фотографию услуги
    pub price: f64,             // Цена за услугу
    pub city_id: i32,           // Идентификатор города, где предоставляется услуга
    pub review_ids: Vec<i32>,   // Идентификаторы отзывов об услуге
}

// Структура NewService используется для создания новых объектов Service
#[derive(Insertable)]
#[table_name = "services"]
pub struct NewService {
    pub organization_id: i32,
    pub description: String,
    pub photo_link: String,
    pub price: f64,
    pub city_id: i32,
    pub review_ids: Vec<i32>,
}
