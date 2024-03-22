use crate::schema;
use diesel::{
    Queryable,
    Insertable,
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
    NullableExpressionMethods,
    PgTextExpressionMethods,
};
use crate::utils::{
    establish_connection,
};
use crate::schema::places;
use serde::{Serialize, Deserialize};
use crate::models::File;


// Структура для таблицы Places 
/*
types
1  предложен
2  одобрен
*/
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct Place {
    pub id:               i32, 
    pub user_id:          i32,
    pub city_id:          Option<i32>,
    pub district_id:      Option<i32>,
    pub region_id:        Option<i32>,
    pub country_id:       i32,
    pub title:            String,
    pub description:      Option<String>,
    pub hours:            Option<String>,
    pub image:            Option<String>,
    pub address:          Option<String>,
    pub count:            i16,
    pub director:         Option<String>,
    pub phone:            Option<String>,
    pub cadastral_number: Option<String>,
    pub cord:             Option<String>,
    pub types:            i32,
    pub created:          chrono::NaiveDateTime,
} 

// Структура для создания новой записи Place
#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "places"] 
pub struct NewPlace {
    pub user_id:          i32,
    pub city_id:          Option<i32>,
    pub district_id:      Option<i32>,
    pub region_id:        Option<i32>,
    pub country_id:       i32,
    pub title:            String,
    pub description:      Option<String>,
    pub hours:            Option<String>,
    pub image:            Option<String>,
    pub address:          Option<String>,
    pub count:            i16,
    pub director:         Option<String>,
    pub phone:            Option<String>,
    pub cadastral_number: Option<String>,
    pub cord:             Option<String>,
    pub types:            i32,
    pub created:          chrono::NaiveDateTime,
}

pub struct SmallPlace {
    pub id:      i32,
    pub title:   String,
    pub address: String,
}

impl Place {
    pub fn search(name: String) -> Vec<Place> {
        return schema::places::table
            .filter(schema::places::ilike("%".to_owned() + &name + "%"))
            .load::<Place>(&_connection)
            .expect("E.");
    }
    pub fn suggested_list() -> Vec<Place> {
        let _connection = establish_connection();
        return schema::places::table
            .filter(schema::places::types.ne(2))
            .load::<Place>(&_connection)
            .expect("E.");
    }
    pub fn count_images(&self) -> usize {
        let _connection = establish_connection();
        return schema::files::table
            .filter(schema::files::object_id.eq(self.id))
            .filter(schema::files::object_types.eq(2))
            .select(schema::files::id)
            .load::<i32>(&_connection)
            .expect("E")
            .len();
    }
    pub fn get_images(&self) -> Vec<File> {
        use crate::schema::files::dsl::files;

        let _connection = establish_connection();
        return files
            .filter(schema::files::object_id.eq(self.id))
            .filter(schema::files::object_types.eq(2))
            .load::<File>(&_connection)
            .expect("E.");
    }
    pub fn get_loc(&self) -> String {
        use crate::schema::places::dsl::places;

        let _connection = establish_connection();
        let mut loc = String::new();
        loc.push_str("Россия, ");
        if self.region_id.is_some() {
            let region_name = schema::regions::table
                .filter(schema::regions::id.eq(self.region_id.unwrap()))
                .select(schema::regions::name)
                .first::<String>(&_connection);
            if region_name.is_ok() {
                loc.push_str(&region_name.expect("E."));
                loc.push_str(", ");
            }
        }
        if self.city_id.is_some() {
            let _name = schema::cities::table
                .filter(schema::cities::id.eq(self.city_id.unwrap()))
                .select(schema::cities::name)
                .first::<String>(&_connection);
            if _name.is_ok() {
                loc.push_str(&_name.expect("E."));
            }
        }
        else if self.district_id.is_some() {
            let _name = schema::districts::table
                .filter(schema::districts::id.eq(self.district_id.unwrap()))
                .select(schema::districts::name)
                .first::<String>(&_connection);
            if _name.is_ok() {
                loc.push_str(&_name.expect("E."));
            }
        }

        return loc;
    }

    pub fn get_image(&self) -> String {
        if self.image.is_some() {
            return self.image.as_deref().unwrap().to_string();
        }
        else {
            return "/static/images/img.jpg".to_string();
        }
    }
    pub fn plus(&self, count: i16) -> () {
        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::places::count.eq(self.count + count))
            .execute(&_connection)
            .expect("Error.");
    }
    pub fn minus(&self, count: i16) -> () {
        let _connection = establish_connection();
        if self.count > 0 {
            diesel::update(self)
                .set(schema::places::count.eq(self.count - count))
                .execute(&_connection)
                .expect("Error.");
        }
    }

    pub fn publish(&self, user_id: i32) -> () {
        let _connection = establish_connection();
        let _user = crate::utils::get_user(user_id).expect("E.");
        if _user.perm > 9 {
            diesel::update(self)
                .set(schema::places::types.eq(2))
                .execute(&_connection)
                .expect("Error.");
            crate::models::Log::create(user_id, self.id, 3, 4);
        }
    }
    pub fn unpublish(&self, user_id: i32) -> () {
        let _connection = establish_connection();
        let _user = crate::utils::get_user(user_id).expect("E.");
        if _user.perm > 9 {
            diesel::update(self)
                .set(schema::places::types.eq(1))
                .execute(&_connection)
                .expect("Error.");
            crate::models::Log::create(user_id, self.id, 3, 8);
        }
    }

    pub fn create (
        user_id:          i32,
        city_id:          Option<i32>,
        district_id:      Option<i32>,
        region_id:        Option<i32>,
        country_id:       i32,
        title:            String,
        description:      Option<String>,
        hours:            Option<String>,
        image:            Option<String>,
        address:          Option<String>,
        director:         Option<String>,
        phone:            Option<String>,
        cadastral_number: Option<String>,
        cord:             Option<String>,
        images:           Vec<String>,
    ) -> i16 {
        use crate::schema::places::dsl::places;

        let _connection = establish_connection();
        let _user = crate::utils::get_user(user_id).expect("E.");
        let types: i32;
        if _user.perm > 9 {
            types = 2;
        } else {
            types = 1;
        }
        
        let new_form = NewPlace {
            user_id:          user_id,
            city_id:          city_id,
            district_id:      district_id,
            region_id:        region_id,
            country_id:       country_id,
            title:            title,
            description:      description,
            hours:            hours,
            image:            image,
            address:          address,
            count:            0,
            director:         director,
            phone:            phone,
            cadastral_number: cadastral_number,
            cord:             cord,
            types:            types,
            created:          chrono::Local::now().naive_utc(),
        };
        let _new = diesel::insert_into(schema::places::table)
            .values(&new_form)
            .get_result::<Place>(&_connection)
            .expect("Error.");

        if images.len() > 0 {
            crate::models::File::create(_new.id, 2, images);
        }
        crate::models::Log::create(user_id, _new.id, 3, 1);

        return 1;
    }
    pub fn edit (  
        &self,
        user_id:          i32,
        city_id:          Option<i32>,
        district_id:      Option<i32>,
        region_id:        Option<i32>,
        country_id:       i32,
        title:            String,
        description:      Option<String>,
        hours:            Option<String>,
        image:            Option<String>,
        address:          Option<String>,
        director:         Option<String>,
        phone:            Option<String>,
        cadastral_number: Option<String>,
        cord:             Option<String>,
        images:           Vec<String>,
    ) -> i16 {
        let _user = crate::utils::get_user(user_id).expect("E.");
        if _user.perm > 9 {
            let _connection = establish_connection();
                diesel::update(self)
                    .set((
                        schema::places::city_id.eq(city_id),
                        schema::places::district_id.eq(district_id),
                        schema::places::region_id.eq(region_id),
                        schema::places::country_id.eq(country_id),
                        schema::places::title.eq(title),
                        schema::places::description.eq(description),
                        schema::places::hours.eq(hours),
                        schema::places::address.eq(address),
                        schema::places::director.eq(director),
                        schema::places::phone.eq(phone),
                        schema::places::cadastral_number.eq(cadastral_number),
                        schema::places::cord.eq(cord),
                    ))
                    .execute(&_connection)
                    .expect("Error.");

            if image.is_some() {
                diesel::update(self)
                    .set(schema::places::image.eq(image))
                    .execute(&_connection)
                    .expect("Error.");
            }
            if images.len() > 0 {
                crate::models::File::create(self.id, 2, images);
            }
            crate::models::Log::create(user_id, self.id, 3, 2);
        }
        return 1;
    }
    pub fn delete(&self, user_id: i32) -> () {
        let _connection = establish_connection();
        let _user = crate::utils::get_user(user_id).expect("E.");
        if _user.perm > 9 {
            let types = match self.types {
                1 => 11,
                2 => 12,
                _ => 12,
            };
            diesel::update(self)
                .set(schema::places::types.eq(types))
                .execute(&_connection)
                .expect("Error.");
            crate::models::Log::create(user_id, self.id, 3, 3);
        }
    }
    pub fn restore(&self, user_id: i32) -> () {
        let _connection = establish_connection();
        let _user = crate::utils::get_user(user_id).expect("E.");
        if _user.perm > 9 {
            let types = match self.types {
                11 => 1,
                12 => 2,
                _  => 2,
            }; 
            diesel::update(self)
                .set(schema::places::types.eq(types))
                .execute(&_connection)
                .expect("Error.");
            crate::models::Log::create(user_id, self.id, 3, 7);
        }
    }

    pub fn country_list(country_id: i32) -> Vec<Place> {
        use crate::schema::places::dsl::places;

        let _connection = establish_connection();
        return places
            .filter(schema::places::country_id.eq(country_id))
            .filter(schema::places::types.eq(2))
            .load::<Place>(&_connection)
            .expect("E.");
    }
    pub fn region_list(region_id: i32) -> Vec<Place> {
        use crate::schema::places::dsl::places;

        let _connection = establish_connection();
        return places
            .filter(schema::places::region_id.eq(region_id))
            .filter(schema::places::types.eq(2))
            .load::<Place>(&_connection)
            .expect("E.");
    }
    pub fn city_list(city_id: i32) -> Vec<Place> {
        use crate::schema::places::dsl::places;

        let _connection = establish_connection();
        return places
            .filter(schema::places::city_id.eq(city_id))
            .filter(schema::places::types.eq(2))
            .load::<Place>(&_connection)
            .expect("E.");
    }
    pub fn get_all() -> Vec<Place> {
        use crate::schema::places::dsl::places;

        let _connection = establish_connection();
        return places
            .filter(schema::places::types.eq(2))
            .load::<Place>(&_connection)
            .expect("E.");
    }
    pub fn search (
        q:        &String,
        limit:    i64,
        offset:   i64,
    ) -> Vec<Place> {
        use crate::schema::places::dsl::places;

        let _connection = establish_connection();
        return places
            .filter(schema::places::title.ilike(&q))
            .or_filter(schema::places::description.ilike(&q))
            .or_filter(schema::places::address.ilike(&q))
            .filter(schema::places::types.eq(2))
            .order(schema::places::title.desc())
            .limit(limit)
            .offset(offset)
            .load::<Place>(&_connection)
            .expect("E.");
    }
    pub fn all() -> Vec<Place> {
        use crate::schema::places::dsl::places;
    
        let _connection = establish_connection();
        return places
            .filter(schema::places::types.eq(2))
            .load::<Place>(&_connection)
            .expect("E.");
    }
    pub fn search_ids ( 
        loc: &String,
    ) -> Vec<i32> {
        let _connection = establish_connection();

        let cities_ids = schema::cities::table
            .filter(schema::cities::name.ilike(loc))
            .select(schema::cities::id)
            .load::<i32>(&_connection)
            .expect("E.");
        let districts_ids = schema::districts::table
            .filter(schema::districts::name.ilike(loc))
            .select(schema::districts::id)
            .load::<i32>(&_connection)
            .expect("E.");
        let regions_ids = schema::regions::table
            .filter(schema::regions::name.ilike(loc))
            .select(schema::regions::id)
            .load::<i32>(&_connection)
            .expect("E.");
        
        if districts_ids.len() > 0 {
            return schema::places::table
                .filter(schema::places::district_id.eq_any(districts_ids))
                .filter(schema::places::types.eq(2))
                .select(schema::places::id)
                .load::<i32>(&_connection)
                .expect("E.");
        }
        if regions_ids.len() > 0 {
            return schema::places::table
                .filter(schema::places::region_id.eq_any(regions_ids))
                .filter(schema::places::types.eq(2))
                .select(schema::places::id)
                .load::<i32>(&_connection)
                .expect("E.");
        }
        if cities_ids.len() > 0 {
            return schema::places::table
                .filter(schema::places::city_id.eq_any(cities_ids))
                .filter(schema::places::types.eq(2))
                .select(schema::places::id)
                .load::<i32>(&_connection)
                .expect("E.");
        }

        return Vec::new();
    }
}