// Подключаем необходимые модули
use diesel::prelude::*;
use chrono::NaiveDate;

// Структура для модели отзыва
#[derive(Debug, Queryable)]
pub struct Review {
    pub id: i32,              // Уникальный идентификатор отзыва
    pub service_id: i32,      // Идентификатор услуги, на которую составлен отзыв
    pub user_id: i32,         // Идентификатор автора отзыва
    pub review_date: NaiveDate, // Дата отзыва
    pub review_text: String,  // Текст отзыва
}

// Структура для создания нового отзыва
#[derive(Debug, Insertable)]
#[table_name = "reviews"]
pub struct NewReview {
    pub service_id: i32,      // Идентификатор услуги, на которую составлен отзыв
    pub user_id: i32,         // Идентификатор автора отзыва
    pub review_date: NaiveDate, // Дата отзыва
    pub review_text: String,  // Текст отзыва
}
