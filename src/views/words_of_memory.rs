// Импортируем необходимые модули для работы с Diesel и БД
use diesel::prelude::*;
use chrono::NaiveDateTime;

// Структура для представления данных о комментарии
#[derive(Debug, Queryable)]
pub struct Comment {
    pub id: i32,                // Уникальный идентификатор комментария
    pub deceased_id: i32,       // Идентификатор усопшего, кому предназначен комментарий
    pub user_id: i32,           // Идентификатор автора комментария
    pub comment_text: String,   // Текст комментария
    pub comment_date: NaiveDateTime, // Дата комментария
}

// Структура для создания нового комментария
#[derive(Debug, Insertable)]
#[table_name = "comments"]
pub struct NewComment {
    pub deceased_id: i32,
    pub user_id: i32,
    pub comment_text: String,
    pub comment_date: NaiveDateTime,
}
