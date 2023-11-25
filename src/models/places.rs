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


// Структура для таблицы Places 
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct Place {
    pub id:          i32, 
    pub user_id:     i32,
    pub city_id:     Option<i32>,
    pub district_id: Option<i32>,
    pub region_id:   Option<i32>,
    pub country_id:  i32,
    pub title:       String,
    pub description: Option<String>,
    pub hours:       Option<String>,
    pub image:       Option<String>,
    pub address:     Option<String>,
    pub count:       i16,
    pub director:    Option<String>,
    pub phone:       Option<String>,
    pub lat:         f64,
    pub lon:         f64,
}

// Структура для создания новой записи Place
#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "places"]
pub struct NewPlace {
    pub user_id:     i32,
    pub city_id:     Option<i32>,
    pub district_id: Option<i32>,
    pub region_id:   Option<i32>,
    pub country_id:  i32,
    pub title:       String,
    pub description: Option<String>,
    pub hours:       Option<String>,
    pub image:       Option<String>,
    pub address:     Option<String>,
    pub count:       i16,
    pub director:    Option<String>,
    pub phone:       Option<String>,
    pub lat:         f64,
    pub lon:         f64,
}

// Реализация методов для структуры Place
impl Place {
    pub fn get_loc(&self) -> String {
        use crate::schema::places::dsl::places;

        let _connection = establish_connection();
        let mut loc = String::new();
        loc.push_str("Россия, ");
        let region_name = schema::regions::table
            .filter(schema::regions::id.eq(self.city_id))
            .first(schema::regions::name)
            .first::<String>(&_connection);
        if region_name.is_ok() {
            loc.push_str(region_name.expect("E."));
            loc.push_str(", ");
        }
        if self.city_id.is_some() {
            let _name = schema::cities::table
                .filter(schema::cities::id.eq(self.city_id.unwrap()))
                .select(schema::cities::name)
                .first::<String>(&_connection);
            if _name.is_ok() {
                loc.push_str(_name.expect("E."));
            }
        }
        else if self.district_id.is_some() {
            let _name = schema::districts::table
                .filter(schema::districts::id.eq(self.district_id.unwrap()))
                .select(schema::districts::name)
                .first::<String>(&_connection);
            if _name.is_ok() {
                loc.push_str(_name.expect("E."));
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
    pub fn create (
        user_id:     i32,
        city_id:     Option<i32>,
        district_id: Option<i32>,
        region_id:   Option<i32>,
        country_id:  i32,
        title:       String,
        description: Option<String>,
        hours:       Option<String>,
        image:       Option<String>,
        address:     Option<String>,
        director:    Option<String>,
        phone:       Option<String>,
        lat:         f64,
        lon:         f64,
    ) -> i16 {
        use crate::schema::places::dsl::places;

        let _connection = establish_connection();
        let _user = crate::utils::get_user(user_id).expect("E.");
        if _user.perm > 10 {
            let new_form = NewPlace {
                user_id:     user_id,
                city_id:     city_id,
                district_id: district_id,
                region_id:   region_id,
                country_id:  country_id,
                title:       title,
                description: description,
                hours:       hours,
                image:       image,
                address:     address,
                count:       0,
                director:    director,
                phone:       phone,
                lat:         lat,
                lon:         lon,
            };
            diesel::insert_into(schema::places::table)
                .values(&new_form)
                .execute(&_connection)
                .expect("Error.");
        }
        return 1;
    }
    pub fn edit ( 
        &self,
        user_id:     i32,
        city_id:     Option<i32>,
        district_id: Option<i32>,
        region_id:   Option<i32>,
        country_id:  i32,
        title:       String,
        description: Option<String>,
        hours:       Option<String>,
        image:       Option<String>,
        address:     Option<String>,
        director:    Option<String>,
        phone:       Option<String>,
        lat:         f64,
        lon:         f64,
    ) -> i16 {
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
                    schema::places::image.eq(image),
                    schema::places::address.eq(address),
                    schema::places::director.eq(director),
                    schema::places::phone.eq(phone),
                    schema::places::lat.eq(lat),
                    schema::places::lon.eq(lon),
                ))
                .execute(&_connection)
                .expect("Error.");
        
        return 1;
    }
    pub fn delete(&self) -> i16 {
        use crate::schema::places::dsl::places;

        let _connection = establish_connection();
        diesel::delete(places.filter(schema::places::id.eq(self.id)))
            .execute(&_connection)
            .expect("E");
        
        return 1;
    }
    pub fn country_list(country_id: i32) -> Vec<Place> {
        use crate::schema::places::dsl::places;

        let _connection = establish_connection();
        return places
            .filter(schema::places::country_id.eq(country_id))
            .load::<Place>(&_connection)
            .expect("E.");
    }
    pub fn region_list(region_id: i32) -> Vec<Place> {
        use crate::schema::places::dsl::places;

        let _connection = establish_connection();
        return places
            .filter(schema::places::region_id.eq(region_id))
            .load::<Place>(&_connection)
            .expect("E.");
    }
    pub fn city_list(city_id: i32) -> Vec<Place> {
        use crate::schema::places::dsl::places;

        let _connection = establish_connection();
        return places
            .filter(schema::places::city_id.eq(city_id))
            .load::<Place>(&_connection)
            .expect("E.");
    }
    pub fn get_all() -> Vec<Place> {
        use crate::schema::places::dsl::places;

        let _connection = establish_connection();
        return places
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
                .select(schema::places::id)
                .load::<i32>(&_connection)
                .expect("E.");
        }
        if regions_ids.len() > 0 {
            return schema::places::table
                .filter(schema::places::region_id.eq_any(regions_ids))
                .select(schema::places::id)
                .load::<i32>(&_connection)
                .expect("E.");
        }
        if cities_ids.len() > 0 {
            return schema::places::table
                .filter(schema::places::city_id.eq_any(cities_ids))
                .select(schema::places::id)
                .load::<i32>(&_connection)
                .expect("E.");
        }

        return Vec::new();
    }
}