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


use diesel::{PgConnection, QueryDsl, RunQueryDsl, ExpressionMethods};
use crate::schema::services; // Подключение модуля с описанием схемы БД

impl Service {
    // Метод для создания нового объекта структуры Service.
    pub fn new(
        organization_id: i32,
        description: String,
        photo_link: String,
        price: f64,
        city_id: i32,
        review_ids: Vec<i32>,
    ) -> Self {
        Service {
            id: 0, // При создании нового объекта, id устанавливается на 0 или другое начальное значение
            organization_id,
            description,
            photo_link,
            price,
            city_id,
            review_ids,
        }
    }

    // Метод для поиска объекта по идентификатору.
    pub fn find_by_id(id: i32, connection: &PgConnection) -> Option<Self> {
        services::table
            .filter(services::id.eq(id))
            .first(connection)
            .ok()
    }

    // Метод для получения всех объектов данной структуры.
    pub fn find_all(connection: &PgConnection) -> Vec<Self> {
        services::table
            .load::<Service>(connection)
            .expect("Failed to load services")
    }

    // Метод для обновления существующего объекта.
    pub fn update(&self, connection: &PgConnection) -> Result<(), diesel::result::Error> {
        diesel::update(services::table.filter(services::id.eq(self.id)))
            .set(self)
            .execute(connection)?;
        Ok(())
    }

    // Метод для удаления объекта.
    pub fn delete(&self, connection: &PgConnection) -> Result<(), diesel::result::Error> {
        diesel::delete(services::table.filter(services::id.eq(self.id)))
            .execute(connection)?;
        Ok(())
    }

    // Метод для создания нового объекта на основе данных из структуры NewService.
    pub fn create(new_data: NewService, connection: &PgConnection) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(services::table)
            .values(&new_data)
            .get_result(connection)
    }

    // Метод для поиска объектов по значению определенного поля.
    pub fn find_by_field(field_value: &str, connection: &PgConnection) -> Vec<Self> {
        services::table
            .filter(services::description.eq(field_value))
            .load::<Service>(connection)
            .expect("Failed to load services")
    }

    // Метод для подсчета общего количества объектов данной структуры.
    pub fn count(connection: &PgConnection) -> usize {
        services::table
            .count()
            .get_result(connection)
            .expect("Failed to get count")
    }
}