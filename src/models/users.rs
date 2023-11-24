use crate::schema;
use crate::schema::{
    users,
    countries,
    regions,
    cities,
    districts,
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


#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct User {
    pub id:       i32,
    pub username: String,
    pub email:    String,
    pub password: String,
    pub description: Option<String>,
    pub image:    Option<String>,
    pub perm:     i16,
}
impl User {
    pub fn is_admin(&self) -> bool {
        return self.perm > 10;
    }
    pub fn is_superuser(&self) -> bool {
        return self.perm > 59;
    }
    pub fn create_superuser(user_id: i32) -> i16 {
        let _connection = establish_connection();

        let _u = diesel::update(users::table.filter(users::id.eq(user_id)))
            .set(schema::users::perm.eq(60))
            .execute(&_connection);
        return 1;
    }
    pub fn next_count() -> String { 
        use crate::schema::users::dsl::users;

        let _connection = establish_connection();
        return (users
            .select(schema::users::id)
            .load::<i32>(&_connection)
            .expect("E.")
            .len() + 1)
            .to_string();
    }
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub username: String,
    pub email:    String,
    pub password: String,
    pub description:      Option<String>,
    pub image:    Option<String>,
    pub perm:     i16,
}

#[derive(Debug, Deserialize)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}




#[derive(Queryable, Serialize, Deserialize, Identifiable)]
pub struct Countrie { 
    pub id:           i32,
    pub name:         String,
    pub geo_id:       Option<i32>,
    pub continent_id: Option<i32>,
    pub timezone_id:  Option<i32>,
    pub phone:        Option<String>,
    pub lat:          Option<f64>,
    pub lon:          Option<f64>,
}
impl Countrie {
    pub fn get_all() -> Vec<Countrie> {
        let _connection = establish_connection();
        return schema::countries::table
            .load::<Countrie>(&_connection)
            .expect("E");
    }
    pub fn create ( 
        name: String,
        lat:  Option<f64>,
        lon:  Option<f64>,
    ) -> i16 {
        let _connection = establish_connection();
        let new_form = NewCountrie {
            name:         name,
            geo_id:       None,
            continent_id: None,
            timezone_id:  None,
            phone:        None,
            lat:          lat,
            lon:          lon,
        };
        let _new = diesel::insert_into(schema::countries::table)
            .values(&new_form)
            .execute(&_connection)
            .expect("Error.");
        
        return 1;
    }
    pub fn edit ( 
        &self,
        name: String,
        lat:  Option<f64>,
        lon:  Option<f64>,
    ) -> i16 {
        let _connection = establish_connection();
        diesel::update(self)
            .set((
                schema::countries::name.eq(name),
                schema::countries::lat.eq(lat),
                schema::countries::lon.eq(lon),
            ))
            .execute(&_connection)
            .expect("Error.");
        
        return 1;
    }
    pub fn delete(&self) -> i16 {
        let _connection = establish_connection();
        diesel::delete(schema::countries::table.filter(schema::countries::id.eq(self.id)))
            .execute(&_connection)
            .expect("E");
        
        return 1;
    }
}

#[derive(Deserialize, Insertable)]
#[table_name="countries"]
pub struct NewCountrie { 
    pub name:         String,
    pub geo_id:       Option<i32>,
    pub continent_id: Option<i32>,
    pub timezone_id:  Option<i32>,
    pub phone:        Option<String>,
    pub lat:          Option<f64>,
    pub lon:          Option<f64>,
}

#[derive(Queryable, Serialize, Deserialize, Identifiable)]
pub struct Region { 
    pub id:          i32,
    pub name:        String,
    pub geo_id:      Option<i32>,
    pub country_id:  i32,
    pub timezone_id: Option<i32>,
    pub lat:         Option<f64>,
    pub lon:         Option<f64>, 
}
impl Region {
    pub fn get_country_all(id: i32) -> Vec<Region> {
        let _connection = establish_connection();
        return schema::regions::table
            .filter(schema::regions::country_id.eq(id))
            .load::<Region>(&_connection)
            .expect("E");
    }
    pub fn get_all() -> Vec<Region> {
        let _connection = establish_connection();
        return schema::regions::table
            .load::<Region>(&_connection)
            .expect("E");
    }
    pub fn create (
        country_id: i32,
        name:       String,
        lat:        Option<f64>,
        lon:        Option<f64>,
    ) -> i16 {
        let _connection = establish_connection();
        let new_form = NewRegion {
            name:         name,
            geo_id:       None,
            country_id:   country_id,
            timezone_id:  None,
            lat:          lat,
            lon:          lon,
        };
        let _new = diesel::insert_into(schema::regions::table)
            .values(&new_form)
            .execute(&_connection)
            .expect("Error.");
        
        return 1;
    }
    pub fn edit ( 
        &self,
        country_id: i32,
        name:       String,
        lat:        Option<f64>,
        lon:        Option<f64>,
    ) -> i16 {
        let _connection = establish_connection();
        diesel::update(self)
            .set((
                schema::regions::name.eq(name),
                schema::regions::country_id.eq(country_id),
                schema::regions::lat.eq(lat),
                schema::regions::lon.eq(lon),
            ))
            .execute(&_connection)
            .expect("Error.");
        
        return 1;
    }
    pub fn delete(&self) -> i16 {
        let _connection = establish_connection();
        diesel::delete(schema::regions::table.filter(schema::regions::id.eq(self.id)))
            .execute(&_connection)
            .expect("E");
        
        return 1;
    }
}

#[derive(Deserialize, Insertable)]
#[table_name="regions"]
pub struct NewRegion { 
    pub name:        String,
    pub geo_id:      Option<i32>,
    pub country_id:  i32,
    pub timezone_id: Option<i32>,
    pub lat:         Option<f64>,
    pub lon:         Option<f64>,
}

#[derive(Queryable, Serialize, Deserialize, Identifiable)]
pub struct Citie { 
    pub id:         i32,
    pub name:       String,
    pub geo_id:     Option<i32>,
    pub region_id:  Option<i32>,
    pub country_id: i32,
    pub lat:        Option<f64>,
    pub lon:        Option<f64>,
}
impl Citie {
    pub fn get_all() -> Vec<Citie> {
        let _connection = establish_connection();
        return schema::cities::table
            .load::<Citie>(&_connection)
            .expect("E");
    }
    pub fn get_region_all(id: i32) -> Vec<Citie> {
        let _connection = establish_connection();
        return schema::cities::table
            .filter(schema::cities::region_id.eq(id))
            .load::<Citie>(&_connection)
            .expect("E");
    }
    pub fn get_country_all(id: i32) -> Vec<Citie> {
        let _connection = establish_connection();
        return schema::cities::table
            .filter(schema::cities::country_id.eq(id))
            .load::<Citie>(&_connection)
            .expect("E");
    }
    pub fn create (
        region_id:  Option<i32>,
        country_id: i32,
        name:       String,
        lat:        Option<f64>,
        lon:        Option<f64>,
    ) -> i16 {
        let _connection = establish_connection();
        let new_form = NewCitie {
            name:         name,
            geo_id:       None,
            region_id:    region_id,
            country_id:   country_id,
            lat:          lat,
            lon:          lon,
        };
        let _new = diesel::insert_into(schema::cities::table)
            .values(&new_form)
            .execute(&_connection)
            .expect("Error.");
        
        return 1;
    }
    pub fn edit ( 
        &self,
        region_id:  Option<i32>,
        country_id: i32,
        name:       String,
        lat:        Option<f64>,
        lon:        Option<f64>,
    ) -> i16 {
        let _connection = establish_connection();
        diesel::update(self)
            .set((
                schema::cities::name.eq(name),
                schema::cities::region_id.eq(region_id),
                schema::cities::country_id.eq(country_id),
                schema::cities::lat.eq(lat),
                schema::cities::lon.eq(lon),
            ))
            .execute(&_connection)
            .expect("Error.");
        
        return 1;
    }
    pub fn delete(&self) -> i16 {
        let _connection = establish_connection();
        diesel::delete(schema::cities::table.filter(schema::cities::id.eq(self.id)))
            .execute(&_connection)
            .expect("E");
        
        return 1;
    }
}

#[derive(Deserialize, Insertable)]
#[table_name="cities"]
pub struct NewCitie { 
    pub name:       String,
    pub geo_id:     Option<i32>,
    pub region_id:  Option<i32>,
    pub country_id: i32,
    pub lat:        Option<f64>,
    pub lon:        Option<f64>,
}


#[derive(Queryable, Serialize, Deserialize, Identifiable)]
pub struct District { 
    pub id:         i32,
    pub name:       String,
    pub region_id:  Option<i32>,
    pub country_id: i32,
    pub lat:        Option<f64>,
    pub lon:        Option<f64>,
}
impl District {
    pub fn get_region_all(id: i32) -> Vec<District> {
        let _connection = establish_connection();
        return schema::districts::table
            .filter(schema::districts::region_id.eq(id))
            .load::<District>(&_connection)
            .expect("E");
    }
    pub fn get_country_all(id: i32) -> Vec<District> {
        let _connection = establish_connection();
        return schema::districts::table
            .filter(schema::districts::country_id.eq(id))
            .load::<District>(&_connection)
            .expect("E");
    }
    pub fn get_all() -> Vec<District> {
        let _connection = establish_connection();
        return schema::districts::table
            .load::<District>(&_connection)
            .expect("E");
    }
    pub fn create (
        region_id:  Option<i32>,
        country_id: i32,
        name:       String,
        lat:        Option<f64>,
        lon:        Option<f64>,
    ) -> i16 {
        let _connection = establish_connection();
        let new_form = NewDistrict {
            name:         name,
            region_id:    region_id,
            country_id:   country_id,
            lat:          lat,
            lon:          lon,
        };
        let _new = diesel::insert_into(schema::districts::table)
            .values(&new_form)
            .execute(&_connection)
            .expect("Error.");
        
        return 1;
    }
    pub fn edit ( 
        &self,
        region_id:  Option<i32>,
        country_id: i32,
        name:       String,
        lat:        Option<f64>,
        lon:        Option<f64>,
    ) -> i16 {
        let _connection = establish_connection();
        diesel::update(self)
            .set((
                schema::districts::name.eq(name),
                schema::districts::region_id.eq(region_id),
                schema::districts::country_id.eq(country_id),
                schema::districts::lat.eq(lat),
                schema::districts::lon.eq(lon),
            ))
            .execute(&_connection)
            .expect("Error.");
        
        return 1;
    }
    pub fn delete(&self) -> i16 {
        let _connection = establish_connection();
        diesel::delete(schema::districts::table.filter(schema::districts::id.eq(self.id)))
            .execute(&_connection)
            .expect("E");
        
        return 1;
    }
}

#[derive(Deserialize, Insertable)]
#[table_name="districts"]
pub struct NewDistrict { 
    pub name:       String,
    pub region_id:  Option<i32>,
    pub country_id: i32,
    pub lat:        Option<f64>,
    pub lon:        Option<f64>,
}