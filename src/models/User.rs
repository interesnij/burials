
use diesel::prelude::*;
use serde::{Serialize, Deserialize};

// Структура User с комментариями к полям
#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,                 // Уникальный идентификатор
    pub first_name: String,      // Имя пользователя
    pub last_name: String,       // Фамилия пользователя
    pub middle_name: String,     // Отчество пользователя
    pub email: String,           // Адрес электронной почты
    pub phone_number: String,    // Номер телефона пользователя
    pub description: String,     // Описание пользователя
    pub photo_link: String,      // Ссылка на фотографию пользователя
    pub deceased_ids: Vec<i32>,  // Идентификаторы зарегистрированных усопших
}

// Структура NewUser для создания новых объектов User
#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "users"] // Указываем таблицу в базе данных
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub middle_name: String,
    pub email: String,
    pub phone_number: String,
    pub description: String,
    pub photo_link: String,
    pub deceased_ids: Vec<i32>,
}
