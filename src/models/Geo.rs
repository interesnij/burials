use diesel::{Insertable, Queryable, RunQueryDsl, ExpressionMethods};
use diesel::pg::PgConnection;
use diesel::result::Error;
use crate::schema::countries;

use diesel::prelude::*;
use crate::schema::countries; // Замените на фактический путь к модулю schema

// Структура Countries
#[derive(Queryable, Debug)] // Добавляем Queryable для работы с базой данных и Debug для отладки
pub struct Countries {
    pub id: i32, // Уникальный идентификатор
    pub name: String, // Название страны
    pub geo_id: i32, // Идентификатор географического местоположения
}

// Структура NewCountries для создания новых объектов
#[derive(Insertable, Debug)]
#[table_name = "countries"]
pub struct NewCountries {
    pub name: String, // Название страны
    pub geo_id: i32, // Идентификатор географического местоположения
}


impl Countries {
    // Метод для создания нового объекта структуры
    pub fn new(name: String, geo_id: i32) -> NewCountries {
        NewCountries { name, geo_id }
    }

    // Метод для поиска объекта по идентификатору
    pub fn find_by_id(conn: &PgConnection, id: i32) -> Result<Option<Self>, Error> {
        use crate::schema::countries::dsl::*;

        let result = countries
            .filter(id.eq(id))
            .first::<Self>(conn)
            .optional()?;

        Ok(result)
    }

    // Метод для получения всех объектов данной структуры
    pub fn find_all(conn: &PgConnection) -> Result<Vec<Self>, Error> {
        use crate::schema::countries::dsl::*;

        let result = countries.load::<Self>(conn)?;

        Ok(result)
    }

    // Метод для обновления существующего объекта
    pub fn update(&self, conn: &PgConnection) -> Result<(), Error> {
        use crate::schema::countries::dsl::*;

        diesel::update(countries.filter(id.eq(self.id)))
            .set(self)
            .execute(conn)?;

        Ok(())
    }

    // Метод для удаления объекта
    pub fn delete(&self, conn: &PgConnection) -> Result<(), Error> {
        use crate::schema::countries::dsl::*;

        diesel::delete(countries.filter(id.eq(self.id)))
            .execute(conn)?;

        Ok(())
    }

    // Метод для поиска объектов по значению определенного поля
    pub fn find_by_field(conn: &PgConnection, field_value: i32) -> Result<Vec<Self>, Error> {
        use crate::schema::countries::dsl::*;

        let result = countries
            .filter(geo_id.eq(field_value))
            .load::<Self>(conn)?;

        Ok(result)
    }

    // Метод для подсчета общего количества объектов данной структуры
    pub fn count(conn: &PgConnection) -> Result<usize, Error> {
        use crate::schema::countries::dsl::*;

        let result = countries.count().get_result(conn)?;

        Ok(result)
    }
}


//---------------------------------------------------------------------------------------------------
use crate::schema::regions; // Замените на фактический путь к модулю schema

// Структура Region
#[derive(Queryable, Debug)] // Добавляем Queryable для работы с базой данных и Debug для отладки
pub struct Region {
    pub id: i32, // Уникальный идентификатор
    pub name: String, // Название региона
    pub geo_id: i32, // Идентификатор географического местоположения
    pub country_id: i32, // Идентификатор страны, к которой принадлежит регион
}

// Структура NewRegions для создания новых объектов
#[derive(Insertable, Debug)]
#[table_name = "region"]
pub struct NewRegion {
    pub name: String, // Название региона
    pub geo_id: i32, // Идентификатор географического местоположения
    pub country_id: i32, // Идентификатор страны, к которой принадлежит регион
}


// Реализация методов для структуры Region
impl Region {
    // Метод для создания нового объекта структуры.
    pub fn new(name: String, geo_id: i32, country_id: i32) -> Self {
        Region { id: 0, name, geo_id, country_id }
    }

    // Метод для поиска объекта по идентификатору.
    pub fn find_by_id(id: i32, connection: &PgConnection) -> Result<Option<Self>, Error> {
        region::table.find(id).first(connection).optional()
    }

    // Метод для получения всех объектов данной структуры.
    pub fn find_all(connection: &PgConnection) -> Result<Vec<Self>, Error> {
        region::table.load(connection)
    }

    // Метод для обновления существующего объекта.
    pub fn update(&self, connection: &PgConnection) -> Result<(), Error> {
        diesel::update(region::table.find(self.id))
            .set(self)
            .execute(connection)?;
        Ok(())
    }

    // Метод для удаления объекта.
    pub fn delete(&self, connection: &PgConnection) -> Result<(), Error> {
        diesel::delete(region::table.find(self.id)).execute(connection)?;
        Ok(())
    }

    // Метод для поиска объектов по значению определенного поля.
    pub fn find_by_field(field_value: i32, field_name: &str, connection: &PgConnection) -> Result<Vec<Self>, Error> {
        region::table.filter(diesel::dsl::sql(&format!("{} = {}", field_name, field_value)))
            .load(connection)
    }

    // Метод для подсчета общего количества объектов данной структуры.
    pub fn count(connection: &PgConnection) -> Result<usize, Error> {
        region::table.count().get_result(connection)
    }
}

// Реализация метода для структуры NewRegion
impl NewRegion {
    // Метод для создания нового объекта на основе данных из структуры New.
    pub fn create(new_data: NewRegion, connection: &PgConnection) -> Result<Self, Error> {
        diesel::insert_into(region::table)
            .values(&new_data)
            .get_result(connection)
    }
}
//------------------------------------------------------------------------------------------------------

use crate::schema::citys; // Замените на фактический путь к модулю schema

// Структура City
#[derive(Queryable, Debug)] // Добавляем Queryable для работы с базой данных и Debug для отладки
pub struct City {
    pub id: i32, // Уникальный идентификатор
    pub name: String, // Название города
    pub geo_id: i32, // Идентификатор географического местоположения
    pub region_id: i32, // Идентификатор региона, к которому принадлежит город
    pub country_id: i32, // Идентификатор страны, к которой принадлежит город
    pub lat: f64, // Широта местоположения
    pub lon: f64, // Долгота местоположения
}

// Структура NewCity для создания новых объектов
#[derive(Insertable, Debug)]
#[table_name = "citys"]
pub struct NewCity {
    pub name: String, // Название города
    pub geo_id: i32, // Идентификатор географического местоположения
    pub region_id: i32, // Идентификатор региона, к которому принадлежит город
    pub country_id: i32, // Идентификатор страны, к которой принадлежит город
    pub lat: f64, // Широта местоположения
    pub lon: f64, // Долгота местоположения
}

impl City {
    // Метод для создания нового объекта структуры City
    pub fn new(name: String, geo_id: i32, region_id: i32, country_id: i32, lat: f64, lon: f64) -> Self {
        City {
            id: 0, // Выставляем нулевой ID, так как база данных автоматически генерирует ID
            name,
            geo_id,
            region_id,
            country_id,
            lat,
            lon,
        }
    }

    // Метод для поиска объекта по идентификатору
    pub fn find_by_id(id: i32, connection: &PgConnection) -> Result<Option<Self>, Error> {
        city::table.find(id).first(connection).optional()
    }

    // Метод для получения всех объектов данной структуры
    pub fn find_all(connection: &PgConnection) -> Result<Vec<Self>, Error> {
        city::table.load(connection)
    }

    // Метод для обновления существующего объекта
    pub fn update(&self, connection: &PgConnection) -> Result<(), Error> {
        diesel::update(city::table.find(self.id))
            .set(self)
            .execute(connection)?;
        Ok(())
    }

    // Метод для удаления объекта
    pub fn delete(&self, connection: &PgConnection) -> Result<(), Error> {
        diesel::delete(city::table.find(self.id)).execute(connection)?;
        Ok(())
    }

    // Метод для поиска объектов по значению определенного поля (пример по полю name)
    pub fn find_by_field(name: &str, connection: &PgConnection) -> Result<Vec<Self>, Error> {
        city::table.filter(city::name.eq(name)).load(connection)
    }

    // Метод для подсчета общего количества объектов данной структуры
    pub fn count(connection: &PgConnection) -> Result<usize, Error> {
        city::table.count().get_result(connection)
    }
}

impl NewCity {
    // Метод для создания нового объекта на основе данных из структуры NewCitys
    pub fn create(&self, connection: &PgConnection) -> Result<Citys, Error> {
        diesel::insert_into(city::table)
            .values(self)
            .get_result(connection)
    }
}

//------------------------------------------------------------------