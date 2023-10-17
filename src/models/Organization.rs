// Подключение необходимых модулей
use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use crate::schema::*;

// Структура для представления данных об организации
#[derive(Queryable, Serialize, Deserialize)]
pub struct Organization {
    pub id: i32,                     // Уникальный идентификатор организации
    pub name: String,                // Название организации
    pub description: Option<String>,         // Описание организации
    pub director: String,            // Руководитель организации
    pub phone_number: String,        // Номер телефона организации
    pub city_id: i32,                // Идентификатор города организации
    pub region_id: Option<i32>,              // Идентификатор региона организации
    pub country_id: i32,             // Идентификатор страны организации
    pub working_hours: String,       // Время работы организации
    pub website: Option<String>,     // Веб-сайт организации (может быть пустым)
    pub photo_link: Option<String>,  // Ссылка на фотографию организации (может быть пустой)
    pub messenger_links: Option<String>, // Ссылки на мессенджеры организации
}

// Структура для создания новой организации
#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "organizations"]
pub struct NewOrganization {
    pub name: String,
    pub description: String,
    pub director: Option<String>,
    pub phone_number: String,
    pub city_id: i32,
    pub region_id: Option<i32>,
    pub country_id: i32,
    pub working_hours: String,
    pub website: Option<String>,
    pub photo_link: Option<String>,
    pub messenger_links: Option<String>,
}

// Реализация методов для структуры Organization
impl Organization {
    // Метод для создания нового объекта структуры
    pub fn new(
        name: String,
        description: Option<String>,
        director: String,
        phone_number: String,
        city_id: i32,
        region_id: Option<i32>,
        country_id: i32,
        working_hours: String,
        website: Option<String>,
        photo_link: Option<String>,
        messenger_links: Option<String>,
    ) -> Self {
        Organization {
            id: 0,  // При создании нового объекта устанавливаем id в 0
            name,
            description,
            director,
            phone_number,
            city_id,
            region_id,
            country_id,
            working_hours,
            website,
            photo_link,
            messenger_links,
        }
    }

    // Метод для поиска объекта по идентификатору
    pub fn find_by_id(id: i32, connection: &PgConnection) -> Option<Self> {
        use crate::schema::organizations::dsl::*;

        let result = organizations.filter(id.eq(id)).first(connection);

        match result {
            Ok(organization) => Some(organization),
            Err(_) => None,
        }
    }


    // Метод для получения всех объектов данной структуры
    pub fn get_all_organization(connection: &PgConnection) -> Vec<Self> {
        use crate::schema::organizations::dsl::*;

        organizations.load(connection).expect("Failed to load organizations")
    }

    // Метод для обновления существующего объекта
    pub fn update(&self, connection: &PgConnection) -> Result<(), diesel::result::Error> {
        use crate::schema::organizations::dsl::*;

        diesel::update(organizations.filter(id.eq(self.id)))
            .set(self)
            .execute(connection)?;

        Ok(())
    }

    // Метод для удаления объекта
    pub fn delete(&self, connection: &PgConnection) -> Result<(), diesel::result::Error> {
        use crate::schema::organizations::dsl::*;

        diesel::delete(organizations.filter(id.eq(self.id))).execute(connection)?;

        Ok(())
    }

    // Метод для поиска объектов по значению определенного поля
    pub fn find_by_field(field_value: &str, field_name: &str, connection: &PgConnection) -> Vec<Self> {
        use crate::schema::organizations::dsl::*;

        organizations
            .filter(diesel::dsl::sql(&format!("LOWER({}) LIKE LOWER('%{}%')", field_name, field_value)))
            .load(connection)
            .expect("Failed to load organizations")
    }

    // Метод для подсчета общего количества объектов данной структуры
    pub fn count(connection: &PgConnection) -> usize {
        use crate::schema::organizations::dsl::*;

        organizations.count().first(connection).unwrap()
    }
}