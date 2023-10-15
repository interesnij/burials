use diesel::prelude::*;
use chrono::NaiveDate;

// Структура для хранения данных об усопшем
pub struct Deceased {
    pub id: i32,                   // Уникальный идентификатор
    pub place_id: i32,             // Идентификатор места захоронения
    pub first_name: String,        // Имя усопшего
    pub last_name: String,         // Фамилия усопшего
    pub middle_name: Option<String>,// Отчество усопшего (может быть отсутствовать)
    pub birth_date: NaiveDate,     // Дата рождения усопшего
    pub death_date: NaiveDate,     // Дата смерти усопшего
    pub photo_link: Option<String>, // Ссылка на фото усопшего (может быть отсутствовать)
    pub data: Option<String>,       // Дополнительные данные (description) (может быть отсутствовать)
    pub memory_words: Option<String>,// Слова памяти (комментарии) (может быть отсутствовать)
    pub user_id:  i32

}

// Структура для создания новых записей об усопших
pub struct NewDeceased {
    pub place_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub middle_name: Option<String>,
    pub birth_date: NaiveDate,
    pub death_date: NaiveDate,
    pub photo_link: Option<String>,
    pub data: Option<String>,
    pub memory_words: Option<String>
    pub user_id:  i32

}


impl Deceased {
    // Метод для создания нового объекта структуры.
    pub fn new( place_id: i32, first_name: String, last_name: String, 
               middle_name: Option<String>, birth_date: NaiveDate, death_date: NaiveDate, 
               photo_link: Option<String>, data: Option<String>, memory_words: Option<String>, user_id: i32) -> Self {
        Deceased {
            place_id,
            first_name,
            last_name,
            middle_name,
            birth_date,
            death_date,
            photo_link,
            data,
            memory_words,
            user_id,
        }
    }

    // Метод для поиска объекта по идентификатору.
    pub fn find_by_id(id: i32, connection: &PgConnection) -> Option<Self> {
        use crate::schema::deceased::dsl::*;

        let result = deceased.filter(id.eq(id))
            .first(connection)
            .optional()
            .expect("Failed to retrieve Deceased by ID");

        result
    }

    // Метод для получения всех объектов данной структуры.
    pub fn get_all_deceased(connection: &PgConnection) -> Vec<Self> {
        use crate::schema::deceased::dsl::*;

        let results = deceased.load::<Deceased>(connection)
            .expect("Failed to retrieve all Deceased records");

        results
    }

    // Метод для обновления существующего объекта.
    pub fn update(&self, connection: &PgConnection) -> Result<(), diesel::result::Error> {
        use crate::schema::deceased::dsl::*;

        diesel::update(deceased.filter(id.eq(self.id)))
            .set(self)
            .execute(connection)?;

        Ok(())
    }

    // Метод для удаления объекта.
    pub fn delete(&self, connection: &PgConnection) -> Result<(), diesel::result::Error> {
        use crate::schema::deceased::dsl::*;

        diesel::delete(deceased.filter(id.eq(self.id)))
            .execute(connection)?;

        Ok(())
    }

    // Метод для поиска объектов по значению определенного поля.
    pub fn find_by_field(field_value: &str, connection: &PgConnection) -> Vec<Self> {
        use crate::schema::deceased::dsl::*;

        let results = deceased.filter(first_name.eq(field_value).or(last_name.eq(field_value)))
            .load::<Deceased>(connection)
            .expect("Failed to retrieve Deceased by field value");

        results
    }

    // Метод для подсчета общего количества объектов данной структуры.
    pub fn count(connection: &PgConnection) -> usize {
        use crate::schema::deceased::dsl::*;

        let count = deceased.count()
            .get_result(connection)
            .expect("Failed to count Deceased records");

        count
    }
}

impl NewDeceased {
    // Метод для создания нового объекта на основе данных из структуры New.
    pub fn create(new_data: NewDeceased, connection: &PgConnection) -> Result<Deceased, diesel::result::Error> {
        use crate::schema::deceased;

        diesel::insert_into(deceased::table)
            .values(&new_data)
            .get_result(connection)
    }
}