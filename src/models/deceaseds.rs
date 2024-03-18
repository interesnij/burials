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
use chrono::NaiveDate;
use crate::utils::{
    establish_connection,
};
use crate::schema::deceaseds;
use serde::{Serialize, Deserialize};
use crate::models::{Place, File};
use crate::errors::Error;



// Структура для хранения данных об усопшем
/*
types 
1  покойник предложен
2  покойник одобрен
3  покойник помещен на стену памяти

11  удален покойник предложеный
12  удален покойник одобреный
13  удален покойник помещеный на стену памяти
*/ 
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct Deceased {
    pub id:           i32,
    pub user_id:      i32,
    pub place_id:     i32,
    pub first_name:   String,
    pub middle_name:  Option<String>,
    pub last_name:    String, 
    pub birth_date:   NaiveDate,
    pub death_date:   NaiveDate,
    pub image:        Option<String>,
    pub memory_words: Option<String>,
    pub lat:          f64,
    pub lon:          f64,
    pub types:        i32,

}

// Структура для создания новых записей об усопших
#[derive(Deserialize, Insertable)]
#[table_name="deceaseds"] 
pub struct NewDeceased {
    pub user_id:      i32,
    pub place_id:     i32,
    pub first_name:   String,
    pub middle_name:  Option<String>,
    pub last_name:    String,
    pub birth_date:   NaiveDate,
    pub death_date:   NaiveDate,
    pub image:        Option<String>,
    pub memory_words: Option<String>,
    pub lat:          f64,
    pub lon:          f64,
    pub types:        i32,
}

impl Deceased {
    pub fn publish(&self, user_id: i32) -> () {
        let _connection = establish_connection();
        let _user = crate::utils::get_user(user_id).expect("E.");
        if _user.perm > 9 {
            diesel::update(self)
                .set(schema::deceaseds::types.eq(2))
                .execute(&_connection)
                .expect("Error.");
        }
    }
    pub fn unpublish(&self, user_id: i32) -> () {
        let _connection = establish_connection();
        let _user = crate::utils::get_user(user_id).expect("E.");
        if _user.perm > 9 {
            diesel::update(self)
                .set(schema::deceaseds::types.eq(1))
                .execute(&_connection)
                .expect("Error.");
        }
    }
    pub fn wall(&self, user_id: i32) -> () {
        let _connection = establish_connection();
        let _user = crate::utils::get_user(user_id).expect("E.");
        if _user.perm > 9 {
            diesel::update(self)
                .set(schema::deceaseds::types.eq(3))
                .execute(&_connection)
                .expect("Error.");
        }
    }
    pub fn unwall(&self, user_id: i32) -> () {
        let _connection = establish_connection();
        let _user = crate::utils::get_user(user_id).expect("E.");
        if _user.perm > 9 {
            diesel::update(self)
                .set(schema::deceaseds::types.eq(2))
                .execute(&_connection)
                .expect("Error.");
        }
    }
    pub fn delete(&self, user_id: i32) -> () {
        let _connection = establish_connection();
        let _user = crate::utils::get_user(user_id).expect("E.");
        if _user.perm > 9 {
            let types = match self.types {
                1 => 11,
                2 => 12,
                3 => 13,
                _ => 12,
            };
            diesel::update(self)
                .set(schema::deceaseds::types.eq(types))
                .execute(&_connection)
                .expect("Error.");
        }
    }
    pub fn restore(&self, user_id: i32) -> () {
        let _connection = establish_connection();
        let _user = crate::utils::get_user(user_id).expect("E.");
        if _user.perm > 9 {
            let types = match self.types {
                11 => 1,
                12 => 2,
                13 => 3,
                _  => 2,
            };
            diesel::update(self)
                .set(schema::deceaseds::types.eq(types))
                .execute(&_connection)
                .expect("Error.");
        }
    }
    pub fn suggested() -> Vec<Deceased> {
        let _connection = establish_connection();
        return schema::deceaseds::table
            .filter(schema::deceaseds::types.ne(2))
            .load::<Deceased>(&_connection)
            .expect("E."); 
    }
    pub fn count_images(&self) -> usize {
        let _connection = establish_connection();
        return schema::files::table
            .filter(schema::files::object_id.eq(self.id))
            .filter(schema::files::object_types.eq(3))
            .select(schema::files::id)
            .load::<i32>(&_connection)
            .expect("E")
            .len();
    }

    pub fn get_place(&self) -> Result<Place, Error> {
        return crate::utils::get_place(self.place_id);
    }
    pub fn get_full_name(&self) -> String {
        self.first_name.clone() + &" ".to_string() + &self.last_name.clone()
    }
    pub fn get_image(&self) -> String {
        if self.image.is_some() {
            return self.image.as_deref().unwrap().to_string();
        }
        else {
            return "/static/images/img.jpg".to_string();
        }
    }
    pub fn create ( 
        user_id:      i32, 
        place_id:     i32,
        first_name:   String,
        middle_name:  Option<String>,
        last_name:    String,
        birth_date:   String,
        death_date:   String,
        image:        Option<String>,
        memory_words: Option<String>,
        lat:          f64,
        lon:          f64,
        images:       Vec<String>,
    ) -> i16 { 
        use crate::schema::deceaseds::dsl::deceaseds;

        let _connection = establish_connection();
        let _user = crate::utils::get_user(user_id).expect("E.");
        let types: i32;
        if _user.perm > 9 {
            types = 2;
        } else {
            types = 1;
        }

        let new_form = NewDeceased {
            user_id:       user_id,
            place_id:      place_id,
            first_name:    first_name,
            middle_name:   middle_name,
            last_name:     last_name,
            birth_date:    chrono::NaiveDate::parse_from_str(&birth_date, "%Y-%m-%d").unwrap(),
            death_date:    chrono::NaiveDate::parse_from_str(&death_date, "%Y-%m-%d").unwrap(),
            image:         image,
            memory_words:  memory_words,
            lat:           lat,
            lon:           lon,
            types:         types,
        };
        let _new = diesel::insert_into(schema::deceaseds::table)
            .values(&new_form)
            .get_result::<Deceased>(&_connection)
            .expect("Error.");
        let _place = crate::utils::get_place(place_id).expect("E.");
        _place.plus(1);

        crate::models::File::create(_new.id, 3, images);
        
        return 1;
    }
    pub fn edit (  
        &self,
        user_id:      i32,
        first_name:   String,
        middle_name:  Option<String>,
        last_name:    String,
        birth_date:   String,
        death_date:   String,
        image:        Option<String>,
        memory_words: Option<String>,
        lat:          f64,
        lon:          f64,
        images:       Vec<String>,
    ) -> i16 {
        use crate::schema::deceaseds::dsl::deceaseds;

        let _connection = establish_connection();
        let _user = crate::utils::get_user(user_id).expect("E.");
        if _user.perm < 10 {
            return 0;
        }

        diesel::update(self)
            .set((
                schema::deceaseds::first_name.eq(first_name),
                schema::deceaseds::middle_name.eq(middle_name),
                schema::deceaseds::last_name.eq(last_name),
                schema::deceaseds::birth_date.eq(chrono::NaiveDate::parse_from_str(&birth_date, "%Y-%m-%d").unwrap()),
                schema::deceaseds::death_date.eq(chrono::NaiveDate::parse_from_str(&death_date, "%Y-%m-%d").unwrap()),
                schema::deceaseds::memory_words.eq(memory_words),
                schema::deceaseds::lat.eq(lat),
                schema::deceaseds::lon.eq(lon),
            ))
            .execute(&_connection)
            .expect("Error.");
        
        if image.is_some() {
            diesel::update(self)
                .set(schema::deceaseds::image.eq(image))
                .execute(&_connection)
                .expect("Error.");
        }
        if images.len() > 0 {
            //diesel::delete(
            //    schema::files::table.filter(
            //        schema::files::object_id.eq(_new.id),
            //         schema::files::object_types.eq(3),
            //    ))
            //    .execute(&_connection)
            //    .expect("E");

            crate::models::File::create(self.id, 3, images);
        }

        return 1;
    }

    pub fn list (
        place_id: i32,
        limit:  i64,
        offset: i64,
    ) -> Vec<Deceased> {
        use crate::schema::deceaseds::dsl::deceaseds;

        let _connection = establish_connection();
        return deceaseds
            .filter(schema::deceaseds::place_id.eq(place_id))
            .filter(schema::deceaseds::types.eq_any(vec!(2, 3)))
            .order(schema::deceaseds::death_date.desc())
            .limit(limit)
            .offset(offset)
            .load::<Deceased>(&_connection)
            .expect("E.");
    }
    pub fn wall_list (
        limit:  i64,
        offset: i64,
    ) -> Vec<Deceased> {
        use crate::schema::deceaseds::dsl::deceaseds;

        let _connection = establish_connection();
        return deceaseds
            .filter(schema::deceaseds::types.eq(3))
            .limit(limit)
            .offset(offset)
            .load::<Deceased>(&_connection)
            .expect("E.");
    }
    pub fn wall_count() -> usize {
        use crate::schema::deceaseds::dsl::deceaseds;

        let _connection = establish_connection();
        return deceaseds
            .filter(schema::deceaseds::types.eq(3))
            .select(schema::deceaseds::id)
            .load::<i32>(&_connection)
            .expect("E.");
    }
    pub fn main_search ( 
        first_name:       Option<String>,
        middle_name:      Option<String>,
        last_name:        String,
        birth_date:       Option<chrono::NaiveDate>,
        death_date:       Option<chrono::NaiveDate>,
        location:         Option<String>,
        deceadeds_id:     Option<i32>,
        is_veteran:       Option<bool>,
        is_famous:        Option<bool>,
        with_photo:       Option<bool>,
        with_coordinates: Option<bool>,
    ) -> Vec<Deceased> { 
        use crate::schema::deceaseds::dsl::deceaseds;

        let _connection = establish_connection();
        let mut stack = Vec::new();
        let list = deceaseds
            .filter(schema::deceaseds::last_name.ilike("%".to_owned() + &last_name + "%"))
            .load::<Deceased>(&_connection)
            .expect("E.");
        for i in list.into_iter() {
            let mut check_exists = false;
            let mut default = true;

            if first_name.is_some() {
                check_exists = i.first_name.contains(first_name.as_deref().unwrap());
                default = false;
            }
            if middle_name.is_some() && i.middle_name.is_some() {
                let i_middle_name = i.middle_name.as_deref().unwrap();
                check_exists = i_middle_name.contains(middle_name.as_deref().unwrap());
                default = false;
            }
            if birth_date.is_some() {
                check_exists = i.birth_date == birth_date.unwrap();
                default = false;
            }
            if death_date.is_some() {
                check_exists = i.death_date == death_date.unwrap();
                default = false;
            }
            if with_photo.is_some() {
                check_exists == i.image.is_some();
                default = false;
            }

            if location.is_some() {
                let loc = "%".to_owned() + location.as_deref().unwrap() + "%"; 
                let places_ids = crate::models::Place::search_ids(&loc);
                let deceaseds_ids = deceaseds
                    .filter(schema::deceaseds::place_id.eq_any(places_ids))
                    .filter(schema::deceaseds::types.eq_any(vec!(2, 3)))
                    .select(schema::deceaseds::id)
                    .load::<i32>(&_connection)
                    .expect("E.");
                check_exists = deceaseds_ids.iter().any(|a| a==&i.id);
                default = false;
            }


            if check_exists || default {
                stack.push(i);
            }
        }
        return stack;

        if location.is_some() {
            let loc = "%".to_owned() + location.as_deref().unwrap() + "%"; 
            let places_ids = crate::models::Place::search_ids(&loc);
            if birth_date.is_some() && death_date.is_some() {
                return deceaseds
                    .filter(schema::deceaseds::place_id.eq_any(places_ids))
                    .filter(schema::deceaseds::last_name.ilike("%".to_owned() + &last_name + "%"))
                    .filter(schema::deceaseds::birth_date.eq(birth_date.unwrap()))
                    .filter(schema::deceaseds::death_date.eq(death_date.unwrap()))
                    .filter(schema::deceaseds::types.eq_any(vec!(2, 3)))
                    .load::<Deceased>(&_connection)
                    .expect("E.");
            } 
            else if birth_date.is_some() && death_date.is_none() {
                return deceaseds
                    .filter(schema::deceaseds::place_id.eq_any(places_ids))
                    .filter(schema::deceaseds::last_name.ilike("%".to_owned() + &last_name + "%"))
                    .filter(schema::deceaseds::birth_date.eq(birth_date.unwrap()))
                    .filter(schema::deceaseds::types.eq_any(vec!(2, 3)))
                    .load::<Deceased>(&_connection)
                    .expect("E.");
            }
            else if death_date.is_some() && birth_date.is_none() {
                return deceaseds
                    .filter(schema::deceaseds::place_id.eq_any(places_ids))
                    .filter(schema::deceaseds::last_name.ilike("%".to_owned() + &last_name + "%"))
                    .filter(schema::deceaseds::death_date.eq(death_date.unwrap()))
                    .filter(schema::deceaseds::types.eq_any(vec!(2, 3)))
                    .load::<Deceased>(&_connection)
                    .expect("E.");
            }
        }
        
            if birth_date.is_some() && death_date.is_some() {
                return deceaseds
                    .filter(schema::deceaseds::last_name.ilike("%".to_owned() + &last_name + "%"))
                    .filter(schema::deceaseds::birth_date.eq(birth_date.unwrap()))
                    .filter(schema::deceaseds::death_date.eq(death_date.unwrap()))
                    .filter(schema::deceaseds::types.eq_any(vec!(2, 3)))
                    .load::<Deceased>(&_connection)
                    .expect("E.");
            }
            else if birth_date.is_some() && death_date.is_none() {
                return deceaseds
                    .filter(schema::deceaseds::last_name.ilike("%".to_owned() + &last_name + "%"))
                    .filter(schema::deceaseds::birth_date.eq(birth_date.unwrap()))
                    .filter(schema::deceaseds::types.eq_any(vec!(2, 3)))
                    .load::<Deceased>(&_connection)
                    .expect("E.");
            }
            else if death_date.is_some() && birth_date.is_none() {
                return deceaseds
                    .filter(schema::deceaseds::last_name.ilike("%".to_owned() + &last_name + "%"))
                    .filter(schema::deceaseds::death_date.eq(death_date.unwrap()))
                    .filter(schema::deceaseds::types.eq_any(vec!(2, 3)))
                    .load::<Deceased>(&_connection)
                    .expect("E.");
            }
            else {
                return deceaseds
                    .filter(schema::deceaseds::last_name.ilike("%".to_owned() + &last_name + "%"))
                    .filter(schema::deceaseds::types.eq_any(vec!(2, 3)))
                    .load::<Deceased>(&_connection)
                    .expect("E.");
            }
        
        //println!("empty");
        return Vec::new();
    }

    pub fn get_all() -> Vec<Deceased> {
        use crate::schema::deceaseds::dsl::deceaseds;

        let _connection = establish_connection();
        return deceaseds
            .load::<Deceased>(&_connection)
            .expect("E.");
    }
    pub fn get_images(&self) -> Vec<File> {
        use crate::schema::files::dsl::files;

        let _connection = establish_connection();
        return files
            .filter(schema::files::object_id.eq(self.id))
            .filter(schema::files::object_types.eq(3))
            .load::<File>(&_connection)
            .expect("E.");
    }
    pub fn count(place_id: i32) -> usize { 
        use crate::schema::deceaseds::dsl::deceaseds;

        let _connection = establish_connection();
        return deceaseds
            .filter(schema::deceaseds::place_id.eq(place_id))
            .filter(schema::deceaseds::types.eq_any(vec!(2, 3)))
            .select(schema::deceaseds::id)
            .load::<i32>(&_connection)
            .expect("E.")
            .len();
    }
}