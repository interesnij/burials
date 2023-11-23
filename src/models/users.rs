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
        return self.perm > 10 && self.perm < 10;
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

#[derive(Deserialize, Insertable)]
#[table_name="cities"]
pub struct NewCitie { 
    pub name:       String,
    pub region_id:  Option<i32>,
    pub country_id: i32,
    pub lat:        Option<f64>,
    pub lon:        Option<f64>,
}