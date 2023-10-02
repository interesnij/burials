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

use crate::schema::comments; // Здесь подключаем необходимую таблицу из схемы

// Методы для структуры Comment
impl Comment {
    // Метод для создания нового объекта структуры Comment
    pub fn new(id: i32, deceased_id: i32, user_id: i32, comment_text: String, comment_date: NaiveDateTime) -> Self {
        Comment {
            id,
            deceased_id,
            user_id,
            comment_text,
            comment_date,
        }
    }

    // Метод для поиска объекта по идентификатору
    pub fn find_by_id(id: i32) -> Option<Self> {
        use diesel::prelude::*;

        let connection = establish_connection();
        comments::table
            .filter(comments::id.eq(id))
            .first(&connection)
            .ok()
    }

    // Метод для получения всех объектов данной структуры
    pub fn find_all() -> Vec<Self> {
        use diesel::prelude::*;

        let connection = establish_connection();
        comments::table.load::<Comment>(&connection).expect("Error loading comments")
    }

    // Метод для обновления существующего объекта
    pub fn update(&self) -> Result<(), diesel::result::Error> {
        use diesel::prelude::*;

        let connection = establish_connection();
        diesel::update(comments::table.find(self.id))
            .set(self)
            .execute(&connection)?;
        Ok(())
    }

    // Метод для удаления объекта
    pub fn delete(&self) -> Result<(), diesel::result::Error> {
        use diesel::prelude::*;

        let connection = establish_connection();
        diesel::delete(comments::table.find(self.id)).execute(&connection)?;
        Ok(())
    }
}

// Методы для структуры NewComment
impl NewComment {
    // Метод для создания нового объекта на основе данных из структуры NewComment
    pub fn create(new_data: NewComment) -> Result<Comment, diesel::result::Error> {
        use diesel::prelude::*;

        let connection = establish_connection();
        diesel::insert_into(comments::table)
            .values(&new_data)
            .get_result(&connection)
    }

    // Метод для поиска объектов по значению определенного поля
    pub fn find_by_field(field_value: FieldType) -> Vec<Comment> {
        use diesel::prelude::*;

        let connection = establish_connection();
        comments::table
            .filter(comments::field.eq(field_value))
            .load::<Comment>(&connection)
            .expect("Error loading comments by field")
    }

    // Метод для подсчета общего количества объектов данной структуры
    pub fn count() -> usize {
        use diesel::prelude::*;

        let connection = establish_connection();
        comments::table.count().get_result(&connection).expect("Error counting comments")
    }
}