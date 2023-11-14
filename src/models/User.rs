use crate::schema;
use crate::schema::{ 
    users,
    cookie_users,
    cookie_stats,
    stat_pages,
};
use crate::diesel::{
    Queryable,
    Insertable,
    QueryDsl,
    ExpressionMethods,
    RunQueryDsl,
    Connection,
};
use serde::{Serialize, Deserialize};
use crate::utils::establish_connection;
use crate::errors::Error;


#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct User {
    pub id:       i32,
    pub username: String,
    pub email:    String,
    pub password: String,
    pub bio:      Option<String>,
    pub image:    Option<String>,
    pub perm:     i16,
}

impl User {

    //     // Метод для создания нового объекта структуры.
    //     pub fn new( place_id: i32, first_name: String, last_name: String, 
    //         middle_name: Option<String>, birth_date: NaiveDate, death_date: NaiveDate, 
    //         photo_link: Option<String>, data: Option<String>, memory_words: Option<String>, user_id: i32) -> Self {
    // User {
    //     place_id,
    //     first_name,
    //     last_name,
    //     middle_name,
    //     birth_date,
    //     death_date,
    //     photo_link,
    //     data,
    //     memory_words,
    //     user_id,
    // }
    // }

    // Метод для поиска объекта по идентификатору.
    pub fn find_by_id(id: i32, connection: &PgConnection) -> Option<Self> {
    use crate::schema::user::dsl::*;

    let result = user.filter(id.eq(id))
        .first(connection)
        .optional()
        .expect("Failed to retrieve User by ID");

    result
    }



    // Метод для обновления существующего объекта.
    pub fn update(&self, connection: &PgConnection) -> Result<(), diesel::result::Error> {
        use crate::schema::user::dsl::*;

        diesel::update(user.filter(id.eq(self.id)))
            .set(self)
            .execute(connection)?;

        Ok(())
    }

    // Метод для удаления объекта.
    pub fn delete(&self, connection: &PgConnection) -> Result<(), diesel::result::Error> {
        use crate::schema::user::dsl::*;

        diesel::delete(user.filter(id.eq(self.id)))
            .execute(connection)?;

        Ok(())
    }










































    pub fn is_superuser(&self) -> bool {
        return self.perm > 59;
    }
    pub fn create_superuser(user_id: i32) -> Result<(), Error> {
        let _connection = establish_connection();
        _connection.transaction(|| Ok({
            let _u = diesel::update(users::table.filter(users::id.eq(user_id)))
                .set(schema::users::perm.eq(60))
                .execute(&_connection);
        }))
    }
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub username: String,
    pub email:    String,
    pub password: String,
    pub bio:      Option<String>,
    pub image:    Option<String>,
    pub perm:     i16,
}

#[derive(Debug, Deserialize)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}

#[derive(Debug, AsChangeset)]
#[table_name = "users"]
pub struct UserChange {
    pub username: String,
    pub email:    String,
    pub password: String,
    pub bio:      String,
    pub image:    String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionUser {
    pub id:       i32,
    pub username: String,
}

#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct CookieUser {
    pub id:         i32,
    pub ip:         String,
    pub device:     i16,
    pub city_ru:    Option<String>,
    pub city_en:    Option<String>,
    pub region_ru:  Option<String>,
    pub region_en:  Option<String>,
    pub country_ru: Option<String>,
    pub country_en: Option<String>,
    pub height:     f64,
    pub seconds:    i32,
    pub created:    chrono::NaiveDateTime,
}
impl CookieUser {
    pub fn get_users_list(page: i32, limit: i32) -> (Vec<CookieUser>, i32) {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: Vec<CookieUser>;

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * limit + 1;
            object_list = CookieUser::get_users(limit.into(), step.into());
        }
        else {
            have_next = limit + 1;
            object_list = CookieUser::get_users(limit.into(), 0);
        }
        if CookieUser::get_users(1, have_next.into()).len() > 0 {
            next_page_number = page + 1;
        }

        return (object_list, next_page_number);
    }
    pub fn get_users(limit: i64, offset: i64) -> Vec<CookieUser> {
        use crate::schema::cookie_users::dsl::cookie_users;

        let _connection = establish_connection();
        return cookie_users
            .filter(schema::cookie_users::seconds.ne(0))
            .filter(schema::cookie_users::height.ne(0.0))
            .order(schema::cookie_users::created.desc())
            .limit(limit)
            .offset(offset)
            .load::<CookieUser>(&_connection)
            .expect("E.");
    }
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name="cookie_users"]
pub struct NewCookieUser {
    pub ip:         String,
    pub device:     i16,
    pub city_ru:    Option<String>,
    pub city_en:    Option<String>,
    pub region_ru:  Option<String>,
    pub region_en:  Option<String>,
    pub country_ru: Option<String>,
    pub country_en: Option<String>,
    pub height:     f64,
    pub seconds:    i32,
    pub created:    chrono::NaiveDateTime,
}

/////////////////////////
// Шифры посещаемых страниц
// 1 - главная
// 2 - о сайте
// 3 - контакты
// 4 - команда
// 5 - сотрудничество
// 6 - вход
// 7 - регитрация
// 8 - выход
// 9 - вопросы ответы
// 10 - инфо

// 11 - профиль
// 12 - заказы
// 13 - история
// 14 - статистика

// 21 - общий поиск
// 22 - поиск статей блога
// 23 - поиск услуг
// 24 - поиск товаров
// 25 - поиск статей обучающих
// 26 - поиск работ

// 31 - теги
// 32 - тег
// 33 - тег - статьи блога
// 34 - тег - услуги
// 35 - тег - товары
// 36 - тег - статьи обучающие
// 37 - тег - работы
// 38 - тег - Пооощь

// 41 - категории блога
// 42 - категория блога
// 43 - статья блога

// 51 - категории опций
// 52 - категория опций
// 53 - технологии опций
// 54 - технология опций
// 55 - опция

// 61 - категории услуг
// 62 - категория услуг
// 63 - услуга

// 71 - категории товаров
// 72 - категория товаров
// 73 - товар

// 81 - категории обучения
// 82 - категория обучения
// 83 - статья обучения

// 91 - категории работ
// 92 - категория работ
// 93 - работа
////////////////////

#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct CookieStat {
    pub id:       i32,
    pub user_id:  i32,
    pub page:     i16,
    pub link:     String,
    pub title:    String,
    pub height:   f64,
    pub seconds:  i32,
    pub created:  chrono::NaiveDateTime,
    pub template: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryResponse {
    pub id:       i32,
    pub link:     String,
    pub title:    String,
    pub height:   f64,
    pub seconds:  i32,
    pub template: String,
}
