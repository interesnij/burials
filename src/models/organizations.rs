use crate::schema;
use crate::schema::organizations;
use diesel::{
    Queryable,
    Insertable,
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
    NullableExpressionMethods,
    PgTextExpressionMethods,
};
use serde::{Serialize, Deserialize};
use crate::utils::{
    establish_connection,
    DataOrganizationsPlace,
};


// Структура для представления данных об организации
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct Organization {
    pub id:          i32,                     // Уникальный идентификатор организации
    pub name:        String,                // Название организации
    pub description: Option<String>,         // Описание организации
    pub director:    String,            // Руководитель организации
    pub phone:       String,        // Номер телефона организации
    pub hours:       String,       // Время работы организации
    pub website:     Option<String>,     // Веб-сайт организации (может быть пустым)
    pub image:       Option<String>,  // Ссылка на фотографию организации (может быть пустой)
    pub user_id:     i32,
}

// Структура для создания новой организации
#[derive(Serialize, Deserialize)]
#[table_name = "organizations"]
pub struct NewOrganization {
    pub name:        String,                // Название организации
    pub description: Option<String>,         // Описание организации
    pub director:    String,            // Руководитель организации
    pub phone:       String,        // Номер телефона организации
    pub hours:       String,       // Время работы организации
    pub website:     Option<String>,     // Веб-сайт организации (может быть пустым)
    pub image:       Option<String>,  // Ссылка на фотографию организации (может быть пустой)
    pub user_id:     i32,
}

// Реализация методов для структуры Organization
impl Organization {
    pub fn create (
        user_id:     i32,
        name:        String,
        description: Option<String>,
        director:    String,
        phone:       String,
        hours:       String, 
        website:     Option<String>,
        image:       Option<String>,
        places:      Vec<DataOrganizationsPlace>,
    ) -> i16 {
        use crate::schema::organizations::dsl::organizations;

        let _connection = establish_connection();
        let _user = crate::utils::get_user(user_id).expect("E.");
        if _user.perm > 10 {
            let new_form = NewOrganization {
                name:        name,
                description: description,
                director:    director,
                phone:       phone,
                hours:       hours,
                website:     website,
                image:       image,
                user_id:     user_id,
            };
            let _new = diesel::insert_into(schema::organizations::table)
                .values(&new_form)
                .get_result::<Organization>(&_connection)
                .expect("Error.");

            for i in places.into_iter() {
                OrganizationsPlace.create (
                    _new.id, 
                    i.city_id, 
                    i.region_id,
                    i.country_id, 
                    i.lat,
                    i.lon,
                )
            }
        }

        return 1;
    }
    pub fn edit (
        user_id:     i32,
        name:        String,
        description: Option<String>,
        director:    String,
        phone:       String,
        hours:       String,
        website:     Option<String>,
        image:       Option<String>,
        places:      Vec<DataOrganizationsPlace>,
    ) -> i16 {
        use crate::schema::organizations::dsl::organizations;

        let _connection = establish_connection();
        let _user = crate::utils::get_user(user_id).expect("E.");
        let _organization = crate::utils::get_organization(object_id).expect("E.");
        if _user.perm > 10 {

            diesel::update(&_organization)
                .set((
                    schema::organizations::first_name.eq(first_name.clone()),
                    schema::organizations::middle_name.eq(middle_name.clone()),
                    schema::organizations::last_name.eq(last_name.clone()),
                    schema::organizations::birth_date.eq(birth_date.clone()),
                    schema::organizations::death_date.eq(death_date.clone()),
                    schema::organizations::image.eq(image.clone()),
                    schema::organizations::memory_words.eq(memory_words.clone()),
                ))
                .execute(&_connection)
                .expect("Error.");

            diesel::delete(organizations_places.filter(schema::organizations_places::organization_id.eq(_organization.id)))
                .execute(&_connection)
                .expect("E");

            for i in places.into_iter() {
                OrganizationsPlace.create (
                    _organization.id, 
                    i.city_id, 
                    i.region_id,
                    i.country_id, 
                    i.lat,
                    i.lon,
                )
            }
        }
        return 1;
    }
    pub fn delete(user_id: i32, object_id: i32) -> i16 {
        use crate::schema::organizations::dsl::organizations;

        let _connection = establish_connection();
        let _user = crate::utils::get_user(user_id).expect("E.");
        if _user.perm > 10 {
            diesel::delete(organizations.filter(schema::organizations::id.eq(object_id)))
                .execute(&_connection)
                .expect("E");
        }
        return 1;
    }

    pub fn list (
        limit:    i64,
        offset:   i64,
    ) -> Vec<Organization> {
        use crate::schema::organizations::dsl::organizations;

        let _connection = establish_connection();
        return organizations
            .limit(limit)
            .offset(offset)
            .load::<Organization>(&_connection)
            .expect("E.");
    }
    pub fn search (
        q:        &String,
        limit:    i64,
        offset:   i64,
    ) -> Vec<Organization> {
        use crate::schema::organizations::dsl::organizations;

        let _connection = establish_connection();
        return organizations
            .filter(schema::organizations::name.ilike(&q))
            .or_filter(schema::organizations::description.ilike(&q))
            .limit(limit)
            .offset(offset)
            .load::<Organization>(&_connection)
            .expect("E.");
    }

    pub fn get_all (
        limit:  i64,
        offset: i64
    ) -> Vec<Organization> {
        use crate::schema::organizations::dsl::organizations;

        let _connection = establish_connection();
        return organizations
            .limit(limit)
            .offset(offset)
            .load::<Organization>(&_connection)
            .expect("E.");
    }
    pub fn count_all() -> Vec<Organization> {
        use crate::schema::organizations::dsl::organizations;

        let _connection = establish_connection();
        return organizations
            .select(schema::organizations::id)
            .load::<i32>(&_connection)
            .expect("E.")
            .len();
    }
}


// Структура для представления данных об организации
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct OrganizationsPlace {
    pub id:              i32,
    pub organization_id: i32,
    pub city_id:         i32,
    pub region_id:       Option<i32>,
    pub country_id:      i32,
    pub lat:             Option<f64>,
    pub lon:             Option<f64>,
}

// Структура для создания новой организации
#[derive(Serialize, Deserialize)]
#[table_name = "organizations_places"]
pub struct NewOrganizationsPlace {
    pub organization_id: i32,
    pub city_id:         i32,
    pub region_id:       Option<i32>,
    pub country_id:      i32,
    pub lat:             Option<f64>,
    pub lon:             Option<f64>,
}

impl OrganizationsPlace {
    pub fn create (
        _organization_id: i32, 
        city_id:          Option<i32>, 
        region_id:        Option<i32>,
        country_id:       Option<i32>, 
        lat:              Option<f64>,
        lon:              Option<f64>,
    ) -> i16 {
        if city_id.is_none() || country_id.is_none() {
            return 0;
        }
        let new_form = NewOrganizationsPlace {
            organization_id: organization_id,
            city_id:         city_id.unwrap(),
            region_id:       region_id,
            country_id:      country_id.unwrap(),
            lat:             lat,
            lon:             lon,
        };
        let _new = diesel::insert_into(schema::organizations_places::table)
            .values(&new_form)
            .execute(&_connection)
            .expect("Error.");
        return 1;
    }
}