use crate::schema;
use crate::schema::{organizations, organizations_places};
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
};


// Структура для представления данных об организации
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct Organization {
    pub id:          i32,
    pub name:        String,
    pub description: String,
    pub director:    String,
    pub phone:       String,
    pub hours:       String,
    pub website:     Option<String>,
    pub image:       Option<String>,
    pub user_id:     i32,
    pub types:       i16,
}

// Структура для создания новой организации
#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "organizations"]
pub struct NewOrganization {
    pub name:        String,
    pub description: String,
    pub director:    String,
    pub phone:       String,
    pub hours:       String,
    pub website:     Option<String>,
    pub image:       Option<String>,
    pub user_id:     i32,
    pub types:       i16,
}

pub struct PlaceSmall {
    pub name:    String,
    pub address: String,
}


// Реализация методов для структуры Organization
impl Organization {
    pub fn get_image(&self) -> String {
        if self.image.is_some() {
            return self.image.as_deref().unwrap().to_string();
        }
        else {
            return "/static/images/img.jpg".to_string();
        }
    }

    pub fn publish(&self, user_id: i32, types: i16) -> () {
        let _connection = establish_connection();
        let _user = crate::utils::get_user(user_id).expect("E.");
        if _user.perm > 9 {
            diesel::update(self)
                .set(schema::organizations::types.eq(2))
                .execute(&_connection)
                .expect("Error.");
        }
    }
    pub fn unpublish(&self, user_id: i32, types: i16) -> () {
        let _connection = establish_connection();
        let _user = crate::utils::get_user(user_id).expect("E.");
        if _user.perm > 9 {
            diesel::update(self)
                .set(schema::organizations::types.eq(1))
                .execute(&_connection)
                .expect("Error.");
        }
    }

    pub fn create ( 
        user_id:     i32,
        name:        String,
        description: String,
        director:    String,
        phone:       String,
        hours:       String, 
        website:     Option<String>,
        image:       Option<String>,
    ) -> i16 {
        use crate::schema::organizations::dsl::organizations;

        let _connection = establish_connection();
        let _user = crate::utils::get_user(user_id).expect("E.");
        let types: i16;
        if _user.perm > 9 {
            types = 2;
        } else {
            types = 1;
        }

        let new_form = NewOrganization {
            name:        name,
            description: description,
            director:    director,
            phone:       phone,
            hours:       hours,
            website:     website,
            image:       image,
            user_id:     user_id,
            types:       types,
        };
        let _new = diesel::insert_into(schema::organizations::table)
            .values(&new_form)
            .get_result::<Organization>(&_connection)
            .expect("Error.");

        return 1;
    }
    pub fn edit (
        &self,
        user_id:     i32,
        object_id:   i32,
        name:        String,
        description: String,
        director:    String,
        phone:       String,
        hours:       String,
        website:     Option<String>,
        image:       Option<String>,
    ) -> i16 {
        use crate::schema::organizations::dsl::organizations;
        let _user = crate::utils::get_user(user_id).expect("E.");
        if self.user_id == user_id || _user.perm > 9 {
            let _connection = establish_connection();
            diesel::update(self)
                .set(( 
                    schema::organizations::name.eq(name),
                    schema::organizations::description.eq(description),
                    schema::organizations::director.eq(director),
                    schema::organizations::phone.eq(phone),
                    schema::organizations::hours.eq(hours),
                    schema::organizations::website.eq(website),
                    schema::organizations::image.eq(image),
                ))
                .execute(&_connection)
                .expect("Error.");

            diesel::delete(schema::organizations_places::table.filter(schema::organizations_places::organization_id.eq(self.id)))
                .execute(&_connection)
                .expect("E");
            }
        return 1;
    }
    pub fn delete(&self, user_id: i32) -> i16 {
        use crate::schema::organizations::dsl::organizations;

        let _connection = establish_connection();
        let _user = crate::utils::get_user(user_id).expect("E.");
        if self.user_id == user_id || _user.perm > 9 {
        diesel::delete(organizations.filter(schema::organizations::id.eq(self.id)))
            .execute(&_connection)
            .expect("E");
        return 1;
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

    pub fn get_city_organizations(city_id: i32) -> (Vec<Organization>, Vec<PlaceSmall>) {
        use crate::utils::get_organization;
        let mut places_stack = Vec::new();
        let mut org_stack = Vec::new();
        let _connection = establish_connection();
        let places_vec = schema::organizations_places::table
            .filter(schema::organizations_places::city_id.eq(city_id))
            .load::<OrganizationsPlace>(&_connection)
            .expect("E."); 
        for _place in places_vec.iter() {
            let org = get_organization(obj.organization_id).expect("E.");
            places_stack.push(PlaceSmall{
                name: org.name.clone(),
                address: _place.get_loc(),
            });
            if org_stack.iter().any(|&i| i != org) {
                org_stack.push(org);
            }
        }
        
        return (org_stack, places_stack);
    }
    pub fn get_region_organizations(region_id: i32) -> (Vec<Organization>, Vec<PlaceSmall>) {
        use crate::utils::get_organization;
        let mut places_stack = Vec::new();
        let mut org_stack = Vec::new();
        let _connection = establish_connection();
        let places_vec = schema::organizations_places::table
            .filter(schema::organizations_places::region_id.eq(region_id))
            .load::<OrganizationsPlace>(&_connection)
            .expect("E.");
        for _place in places_vec.iter() {
            let org = get_organization(obj.organization_id).expect("E.");
            places_stack.push(PlaceSmall{
                name: org.name.clone(),
                address: _place.get_loc(),
            });
            if org_stack.iter().any(|&i| i != org) {
                org_stack.push(org);
            }
        }
        
        return (org_stack, places_stack);
    }
    pub fn get_country_organizations(country_id: i32) -> (Vec<Organization>, Vec<PlaceSmall>) {
        use crate::utils::get_organization;
        let mut places_stack = Vec::new();
        let mut org_stack = Vec::new();
        let _connection = establish_connection();
        let places_vec = schema::organizations_places::table
            .filter(schema::organizations_places::country_id.eq(country_id))
            .load::<OrganizationsPlace>(&_connection)
            .expect("E.");
        for _place in places_vec.iter() {
            let org = get_organization(obj.organization_id).expect("E.");
            places_stack.push(PlaceSmall{
                name: org.name.clone(),
                address: _place.get_loc(),
            });
            if org_stack.iter().any(|&i| i != org) {
                org_stack.push(org);
            }
        }
        return (org_stack, places_stack);
    }
    pub fn count_all() -> usize {
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
    pub address2:        String,
}

// Структура для создания новой организации
#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "organizations_places"]
pub struct NewOrganizationsPlace {
    pub organization_id: i32,
    pub city_id:         i32,
    pub region_id:       Option<i32>,
    pub country_id:      i32,
    pub address2:        String,
}

impl OrganizationsPlace {
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
                loc.push_str(", ");
            }
        }
        else if self.district_id.is_some() {
            let _name = schema::districts::table
                .filter(schema::districts::id.eq(self.district_id.unwrap()))
                .select(schema::districts::name)
                .first::<String>(&_connection);
            if _name.is_ok() {
                loc.push_str(&_name.expect("E."));
                loc.push_str(", ");
            }
        }
        loc.push_str(self.address2);
        return loc;
    }
    pub fn create (
        user_id:         i32,
        organization_id: i32, 
        city_id:         i32,
        region_id:       Option<i32>,
        country_id:      i32, 
        address2:        String,
    ) -> i16 {
        let _connection = establish_connection();
        let _user = crate::utils::get_user(user_id).expect("E.");
        let _organization = crate::utils::get_organization(organization_id).expect("E.");
        if _organization.user_id == user_id || _user.perm > 9 {
            let new_form = NewOrganizationsPlace {
                organization_id: organization_id,
                city_id:         city_id,
                region_id:       region_id,
                country_id:      country_id,
                address2:        address2,
            };
            let _new = diesel::insert_into(schema::organizations_places::table)
                .values(&new_form)
                .execute(&_connection)
                .expect("Error.");
        }
        return 1;
    }

    pub fn edit (
        &self,
        user_id:  i32,
        address2: String,
    ) -> i16 {
        let _connection = establish_connection();
        let _user = crate::utils::get_user(user_id).expect("E.");
        let _organization = crate::utils::get_organization(self.organization_id).expect("E.");
        if _organization.user_id == user_id || _user.perm > 9 {
            diesel::update(self)
                .set(schema::organizations_places::address2.eq(address2))
                .execute(&_connection)
                .expect("Error.");
        }
        return 1;
    }
    pub fn delete(&self, user_id: i32) -> i16 {
        use crate::schema::organizations_places::dsl::organizations_places;

        let _connection = establish_connection();
        let _user = crate::utils::get_user(user_id).expect("E.");
        let _organization = crate::utils::get_organization(self.organization_id).expect("E.");
        if _organization.user_id == user_id || _user.perm > 9 {
            diesel::delete(organizations_places.filter(schema::organizations_places::id.eq(self.id)))
                .execute(&_connection)
                .expect("E");
        }
        return 1;
    }

}