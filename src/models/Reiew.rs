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


use crate::schema::reviews; // Подключаем модуль схемы таблицы reviews
use diesel::prelude::*;
use diesel::result::Error;
use chrono::NaiveDate;

impl Review {
    // Метод для создания нового объекта структуры Review
    pub fn new(service_id: i32, user_id: i32, review_date: NaiveDate, review_text: String) -> Self {
        Review {
            id: 0, // Идентификатор будет автоматически сгенерирован в базе данных
            service_id,
            user_id,
            review_date,
            review_text,
        }
    }

    // Метод для поиска объекта по идентификатору
    pub fn find_by_id(id: i32, connection: &PgConnection) -> Option<Self> {
        reviews::table.find(id)
            .first(connection)
            .ok()
    }

    // Метод для получения всех объектов данной структуры
    pub fn find_all(connection: &PgConnection) -> Vec<Self> {
        reviews::table.load::<Review>(connection)
            .expect("Failed to load reviews")
    }

    // Метод для обновления существующего объекта
    pub fn update(&self, connection: &PgConnection) -> Result<(), Error> {
        diesel::update(reviews::table.find(self.id))
            .set(self)
            .execute(connection)?;
        Ok(())
    }

    // Метод для удаления объекта
    pub fn delete(&self, connection: &PgConnection) -> Result<(), Error> {
        diesel::delete(reviews::table.find(self.id))
            .execute(connection)?;
        Ok(())
    }

    // Метод для поиска объектов по значению определенного поля
    pub fn find_by_field(field_value: &str, connection: &PgConnection) -> Vec<Self> {
        reviews::table.filter(reviews::review_text.eq(field_value))
            .load::<Review>(connection)
            .expect("Failed to load reviews")
    }

    // Метод для подсчета общего количества объектов данной структуры
    pub fn count(connection: &PgConnection) -> usize {
        reviews::table.count()
            .first(connection)
            .unwrap_or(0)
    }
}

impl NewReview {
    // Метод для создания нового объекта на основе данных из структуры NewReview
    pub fn create(new_data: NewReview, connection: &PgConnection) -> Result<Self, Error> {
        diesel::insert_into(reviews::table)
            .values(&new_data)
            .get_result(connection)
    }
}