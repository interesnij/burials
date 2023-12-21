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
use crate::models::Place;
use crate::errors::Error;



// Структура для хранения данных об усопшем
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
}

impl Deceased {
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
        println!("birth_date {:?}", birth_date);
        println!("death_date {:?}", death_date);
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
        if images.len() > 1 {
            //diesel::delete(
            //    schema::files::table.filter(
            //        schema::files::object_id.eq(_new.id),
            //         schema::files::object_types.eq(3),
            //    ))
            //    .execute(&_connection)
            //    .expect("E");

            crate::models::File::create(_new.id, 3, images);
        }

        return 1;
    }
    pub fn delete(&self) -> i16 {
        use crate::schema::deceaseds::dsl::deceaseds;

        let _connection = establish_connection();
        diesel::delete(deceaseds.filter(schema::deceaseds::id.eq(self.id)))
            .execute(&_connection)
            .expect("E");

        let _place = crate::utils::get_place(self.place_id).expect("E.");
        _place.minus(1);
        
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
            .order(schema::deceaseds::death_date.desc())
            .limit(limit)
            .offset(offset)
            .load::<Deceased>(&_connection)
            .expect("E.");
    }
    pub fn main_search ( 
        first_name:  String,
        middle_name: Option<String>,
        last_name:   String,
        birth_date:  Option<chrono::NaiveDate>,
        death_date:  Option<chrono::NaiveDate>,
        location:    Option<String>,
        //limit:       i64,
        //offset:      i64,
    ) -> Vec<Deceased> { 
        use crate::schema::deceaseds::dsl::deceaseds;

        let _connection = establish_connection();
        let middle: String;
        if middle_name.is_some() {
            middle = "%".to_owned() + middle_name.as_deref().unwrap() + "%";
        } 
        else { 
            middle = "%".to_owned() + "" + "%";
        }
        if location.is_some() {
            println!("location exists!!");
            let loc = "%".to_owned() + location.as_deref().unwrap() + "%"; 
            let places_ids = crate::models::Place::search_ids(&loc);
            if birth_date.is_some() && death_date.is_some() {
                println!("location && birth_date && death_date");
                return deceaseds
                    .filter(schema::deceaseds::place_id.eq_any(places_ids))
                    .filter(schema::deceaseds::last_name.ilike("%".to_owned() + &last_name + "%"))
                    .filter(schema::deceaseds::first_name.ilike("%".to_owned() + &first_name + "%"))
                    //.or_filter(schema::deceaseds::middle_name.ilike(middle))
                    
                    .filter(schema::deceaseds::birth_date.eq(birth_date.unwrap()))
                    .filter(schema::deceaseds::death_date.eq(death_date.unwrap()))
                    //.limit(limit)
                    //.offset(offset)
                    .load::<Deceased>(&_connection)
                    .expect("E.");
            } 
            else if birth_date.is_some() && death_date.is_none() {
                println!("location && birth_date");
                return deceaseds
                    .filter(schema::deceaseds::place_id.eq_any(places_ids))
                    .filter(schema::deceaseds::last_name.ilike("%".to_owned() + &last_name + "%"))
                    .filter(schema::deceaseds::first_name.ilike("%".to_owned() + &first_name + "%"))
                    .filter(schema::deceaseds::birth_date.eq(birth_date.unwrap()))
                    //.or_filter(schema::deceaseds::middle_name.ilike(middle))
                    //.limit(limit)
                    //.offset(offset)
                    .load::<Deceased>(&_connection)
                    .expect("E.");
            }
            else if death_date.is_some() && birth_date.is_none() {
                println!("location && death_date");
                return deceaseds
                    .filter(schema::deceaseds::place_id.eq_any(places_ids))
                    .filter(schema::deceaseds::last_name.ilike("%".to_owned() + &last_name + "%"))
                    .filter(schema::deceaseds::first_name.ilike("%".to_owned() + &first_name + "%"))
                    //.or_filter(schema::deceaseds::middle_name.ilike(middle))
                    .filter(schema::deceaseds::death_date.eq(death_date.unwrap()))
                    //.limit(limit)
                    //.offset(offset)
                    .load::<Deceased>(&_connection)
                    .expect("E.");
            }
        }
        
            if birth_date.is_some() && death_date.is_some() {
                println!("birth_date && death_date");
                return deceaseds
                    .filter(schema::deceaseds::last_name.ilike("%".to_owned() + &last_name + "%"))
                    .filter(schema::deceaseds::first_name.ilike("%".to_owned() + &first_name + "%"))
                    //.or_filter(schema::deceaseds::middle_name.ilike(middle))
                    .filter(schema::deceaseds::birth_date.eq(birth_date.unwrap()))
                    .filter(schema::deceaseds::death_date.eq(death_date.unwrap()))
                    //.limit(limit)
                    //.offset(offset)
                    .load::<Deceased>(&_connection)
                    .expect("E.");
            }
            else if birth_date.is_some() && death_date.is_none() {
                println!("birth_date");
                return deceaseds
                    .filter(schema::deceaseds::last_name.ilike("%".to_owned() + &last_name + "%"))
                    .filter(schema::deceaseds::first_name.ilike("%".to_owned() + &first_name + "%"))
                    //.or_filter(schema::deceaseds::middle_name.ilike(middle))
                    .filter(schema::deceaseds::birth_date.eq(birth_date.unwrap()))
                    //.limit(limit)
                    //.offset(offset)
                    .load::<Deceased>(&_connection)
                    .expect("E.");
            }
            else if death_date.is_some() && birth_date.is_none() {
                println!("death_date");
                return deceaseds
                    .filter(schema::deceaseds::last_name.ilike("%".to_owned() + &last_name + "%"))
                    //.or_filter(schema::deceaseds::middle_name.ilike(middle))
                    .or_filter(schema::deceaseds::first_name.ilike("%".to_owned() + &first_name + "%"))
                    .filter(schema::deceaseds::death_date.eq(death_date.unwrap()))
                    //.limit(limit)
                    //.offset(offset)
                    .load::<Deceased>(&_connection)
                    .expect("E.");
            }
            else {
                println!("default!!");
                return deceaseds
                    .filter(schema::deceaseds::last_name.ilike("%".to_owned() + &last_name + "%"))
                    //.or_filter(schema::deceaseds::middle_name.ilike(middle))
                    .filter(schema::deceaseds::first_name.ilike("%".to_owned() + &first_name + "%"))
                    //.limit(limit)
                    //.offset(offset)
                    .load::<Deceased>(&_connection)
                    .expect("E.");
            }
        
        println!("empty");
        return Vec::new();
    }

    // Метод для получения всех объектов данной структуры.
    pub fn get_all() -> Vec<Deceased> {
        use crate::schema::deceaseds::dsl::deceaseds;

        let _connection = establish_connection();
        return deceaseds
            .load::<Deceased>(&_connection)
            .expect("E.");
    }
    pub fn count(place_id: i32) -> usize { 
        use crate::schema::deceaseds::dsl::deceaseds;

        let _connection = establish_connection();
        return deceaseds
            .filter(schema::deceaseds::place_id.eq(place_id))
            .select(schema::deceaseds::id)
            .load::<i32>(&_connection)
            .expect("E.")
            .len();
    }
}