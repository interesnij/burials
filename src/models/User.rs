
use diesel::prelude::*;
use serde::{Serialize, Deserialize};

// Структура User 
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

impl User {
    // Метод для создания нового объекта структуры User
    pub fn new(
        first_name: String,
        last_name: String,
        middle_name: String,
        email: String,
        phone_number: String,
        description: String,
        photo_link: String,
        deceased_ids: Vec<i32>,
    ) -> Self {
        User {
            id: 0, // Идентификатор будет установлен базой данных при вставке
            first_name,
            last_name,
            middle_name,
            email,
            phone_number,
            description,
            photo_link,
            deceased_ids,
        }
    }

    // Метод для поиска объекта по идентификатору
    pub fn find_by_id(id: i32, conn: &PgConnection) -> QueryResult<Option<Self>> {
        use crate::schema::users::dsl::*;

        users.filter(id.eq(id)).first(conn).optional()
    }

    // Метод для получения всех объектов данной структуры
    pub fn find_all(conn: &PgConnection) -> QueryResult<Vec<Self>> {
        use crate::schema::users::dsl::*;

        users.load(conn)
    }

    // Метод для обновления существующего объекта
    pub fn update(&self, conn: &PgConnection) -> QueryResult<()> {
        use crate::schema::users::dsl::*;

        diesel::update(users.filter(id.eq(self.id)))
            .set(self)
            .execute(conn)?;

        Ok(())
    }

    // Метод для удаления объекта
    pub fn delete(&self, conn: &PgConnection) -> QueryResult<()> {
        use crate::schema::users::dsl::*;

        diesel::delete(users.filter(id.eq(self.id))).execute(conn)?;

        Ok(())
    }

    // Метод для поиска объектов по значению определенного поля
    pub fn find_by_field(field_value: &str, conn: &PgConnection) -> QueryResult<Vec<Self>> {
        use crate::schema::users::dsl::*;

        users
            .filter(first_name.eq(field_value)
            .or(last_name.eq(field_value))
            .or(middle_name.eq(field_value))
            .or(email.eq(field_value))
            .or(phone_number.eq(field_value))
            .or(description.eq(field_value))
            .or(photo_link.eq(field_value)))
            .load(conn)
    }

    // Метод для подсчета общего количества объектов данной структуры
    pub fn count(conn: &PgConnection) -> QueryResult<usize> {
        use crate::schema::users::dsl::*;

        users.count().get_result(conn)
    }
}

impl NewUser {
    // Метод для создания нового объекта на основе данных из структуры NewUser
    pub fn create(new_data: NewUser, conn: &PgConnection) -> QueryResult<Self> {
        use crate::schema::users::dsl::*;

        diesel::insert_into(users)
            .values(&new_data)
            .get_result(conn)
    }
}