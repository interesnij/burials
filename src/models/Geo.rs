use diesel::{Insertable, Queryable, RunQueryDsl, ExpressionMethods};
use diesel::pg::PgConnection;
use diesel::result::Error;
use crate::schema;
use crate::schema::{
    countries,
    regions,
    cities,
};
use crate::utils::{
    establish_connection,
};


#[derive(Queryable, Serialize, Deserialize, Identifiable)]
pub struct Countrie { 
    pub id:           i32,
    pub name:         String,
    pub name_ru:      Option<String>,
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
    pub name_ru:      Option<String>,
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
    pub name_ru:     Option<String>,
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
    pub name_ru:     Option<String>,
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
    pub name_ru:    Option<String>,
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
    pub name_ru:    Option<String>,
    pub geo_id:     Option<i32>,
    pub region_id:  Option<i32>,
    pub country_id: i32,
    pub lat:        Option<f64>,
    pub lon:        Option<f64>,
}