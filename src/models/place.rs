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
    pub place_id: i32,                    // Идентификатор покойников
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
    pub place_id: i32,                    // Идентификатор покойников
    pub organization_id: i32,                // Идентификатор организаций
    pub burial_count: i32,                   // Количество захоронений
    pub cemetery_director: Option<String>,    // Руководитель кладбища
    pub cemetery_phone_number: Option<String>,// Номер телефона кладбища
}


use diesel::{prelude::*, result::Error};
use crate::schema::places; // Подключение модуля для таблицы Places

// Реализация методов для структуры Place

impl Place {


    pub fn get_place (
        limit:    i64,
        offset:   i64,
        is_admin: bool
    ) -> Vec<Place> {
        use crate::schema::place::dsl::place;

        let _connection = establish_connection();
        if is_admin {
             return place
                .order(schema::place::cemetery_name.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::place::id,
                    schema::place::city_id,
                    schema::place::region_id,
                    schema::place::country_id,
                    schema::place::cemetery_name,
                    schema::place::description.nullable(),
                    schema::place::summer_hours.nullable(),
                    schema::place::winter_hours.nullable(),
                    schema::place::photo_links.nullable(),
                    schema::place::address.nullable(),
                    schema::place::place_id,
                    schema::place::organization_id,
                    schema::place::burial_count,
                    schema::place::cemetery_director.nullable(),
                    schema::place::cemetery_phone_number.nullable(),
                ))
                .load::<Place>(&_connection)
                .expect("E.");
        } else {
            return place
                .order(schema::place::cemetery_name.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::place::id,
                    schema::place::city_id,
                    schema::place::region_id,
                    schema::place::country_id,
                    schema::place::cemetery_name,
                    schema::place::description.nullable(),
                    schema::place::summer_hours.nullable(),
                    schema::place::winter_hours.nullable(),
                    schema::place::photo_links.nullable(),
                    schema::place::address.nullable(),
                    schema::place::place_id,
                    schema::place::organization_id,
                    schema::place::burial_count,
                    schema::place::cemetery_director.nullable(),
                    schema::place::cemetery_phone_number.nullable(),
                ))
                .load::<Place>(&_connection)
                .expect("E.");
        }
    }
    pub fn search_place (
        q:        &String,
        limit:    i64,
        offset:   i64,
        is_admin: bool
    ) -> Vec<Place> {
        use crate::schema::place::dsl::place;

        let _connection = establish_connection();
        if is_admin {
             return place
                .filter(schema::place::cemetery_name.ilike(&q))
                .or_filter(schema::place::description.ilike(&q))
                .or_filter(schema::place::address.ilike(&q))
                .or_filter(schema::place::cemetery_director.ilike(&q))
                .order(schema::place::cemetery_name.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::place::id,
                    schema::place::city_id,
                    schema::place::region_id,
                    schema::place::country_id,
                    schema::place::cemetery_name,
                    schema::place::description.nullable(),
                    schema::place::summer_hours.nullable(),
                    schema::place::winter_hours.nullable(),
                    schema::place::photo_links.nullable(),
                    schema::place::address.nullable(),
                    schema::place::place_id,
                    schema::place::organization_id,
                    schema::place::burial_count,
                    schema::place::cemetery_director.nullable(),
                    schema::place::cemetery_phone_number.nullable(),
                ))
                .load::<Place>(&_connection)
                .expect("E.");
        } else {
            return place
                .filter(schema::place::cemetery_name.ilike(&q))
                .or_filter(schema::place::description.ilike(&q))
                .or_filter(schema::place::address.ilike(&q))
                .or_filter(schema::place::cemetery_director.ilike(&q))
                .order(schema::place::cemetery_name.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::place::id,
                    schema::place::city_id,
                    schema::place::region_id,
                    schema::place::country_id,
                    schema::place::cemetery_name,
                    schema::place::description.nullable(),
                    schema::place::summer_hours.nullable(),
                    schema::place::winter_hours.nullable(),
                    schema::place::photo_links.nullable(),
                    schema::place::address.nullable(),
                    schema::place::place_id,
                    schema::place::organization_id,
                    schema::place::burial_count,
                    schema::place::cemetery_director.nullable(),
                    schema::place::cemetery_phone_number.nullable(),
                ))
                .load::<Place>(&_connection)
                .expect("E.");
        }
    }
















    // Метод для создания нового объекта структуры.
    pub fn new(
        city_id: i32,
        region_id: i32,
        country_id: i32,
        cemetery_name: String, 
        description: Option<String>,
        summer_hours: Option<String>,
        winter_hours: Option<String>,
        photo_links: Option<String>,
        address: Option<String>,
        place_id: i32,
        organization_id: i32,
        burial_count: i32,
        cemetery_director: Option<String>,
        cemetery_phone_number: Option<String>,
    ) -> NewPlace {
        NewPlace {
            city_id,
            region_id,
            country_id,
            cemetery_name,
            description,
            summer_hours,
            winter_hours,
            photo_links,
            address,
            place_id,
            organization_id,
            burial_count,
            cemetery_director,
            cemetery_phone_number,
        }
    }

    // Метод для поиска объекта по идентификатору.
    pub fn find_by_id(id: i32, connection: &PgConnection) -> Result<Option<Self>, Error> {
        places::table.find(id).first(connection).optional()
    }

    // Метод для получения всех объектов данной структуры.get_all_place
    pub fn get_all_place(connection: &PgConnection) -> Result<Vec<Self>, Error> {
        places::table.load::<Place>(connection)
    }

    // Метод для обновления существующего объекта.
    pub fn update(&self, connection: &PgConnection) -> Result<(), Error> {
        diesel::update(places::table.find(self.id))
            .set(self)
            .execute(connection)?;
        Ok(())
    }

    // Метод для удаления объекта.
    pub fn delete(&self, connection: &PgConnection) -> Result<(), Error> {
        diesel::delete(places::table.find(self.id)).execute(connection)?;
        Ok(())
    }

    // Метод для поиска объектов по значению определенного поля.
    pub fn find_by_field(field_value: String, field_name: &str, connection: &PgConnection) -> Result<Vec<Self>, Error> {
        places::table.filter(places::dsl::sql(field_name).ilike(format!("%{}%", field_value)))
            .load::<Place>(connection)
    }

    // Метод для подсчета общего количества объектов данной структуры.
    pub fn count(connection: &PgConnection) -> Result<usize, Error> {
        places::table.count().get_result(connection)
    }
}