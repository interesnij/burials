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
use crate::schema::services;
use crate::utils::{
    establish_connection,
};
use serde::{Serialize, Deserialize};


#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct Service {
    pub id:       i32,
    pub title:    String,
    pub position: i16,
} 

// Структура NewService используется для создания новых объектов Service
#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "services"]
pub struct NewService {
    pub title:    String,
    pub position: i16,
}

impl Service {
    pub fn create (
        user_id:  i32,
        title:    String,
        position: i16,
    ) -> i16 {
        let _user = crate::utils::get_user(user_id).expect("E.");
        if _user.perm < 10 {
            return 0;
        }
        let _connection = establish_connection();
        let new_form = NewService {
            title:    title,
            position: position,
        };
        diesel::insert_into(schema::services::table)
            .values(&new_form)
            .execute(&_connection)
            .expect("Error.");
        return 1;
    }
    pub fn edit (
        &self,
        user_id:  i32,
        title:    String,
        position: i16,
    ) -> i16 { 
        let _user = crate::utils::get_user(user_id).expect("E.");
        if _user.perm < 10 {
            return 0;
        }

        diesel::update(self)
            .set((
                schema::services::title.eq(title),
                schema::services::position.eq(position),
            ))
            .execute(&_connection)
            .expect("Error.");
        return 1;
    }
    pub fn delete(&self, user_id: i32) -> i16 {
        let _user = crate::utils::get_user(user_id).expect("E.");
        if _user.perm < 10 {
            return 0;
        }
        let _connection = establish_connection();            
        diesel::delete(services.filter(schema::services::id.eq(self.id)))
            .execute(&_connection)
            .expect("E");
        return 1;
    }

    pub fn list (
        organization_id: i32,
        limit:           i64,
        offset:          i64,
    ) -> Vec<Service> {
        use crate::schema::services::dsl::services;

        let _connection = establish_connection();
        return services
            .filter(schema::services::organization_id.eq(organization_id))
            .order(schema::services::price.asc())
            .limit(limit)
            .offset(offset)
            .load::<Service>(&_connection)
            .expect("E.");
    }
    pub fn search (
        organization_id: i32,
        q:               &String,
        limit:           i64,
        offset:          i64,
    ) -> Vec<Service> {
        use crate::schema::services::dsl::services;

        let _connection = establish_connection();
        return services
            .filter(schema::services::organization_id.eq(organization_id))
            .filter(schema::services::title.ilike(&q))
            .filter(schema::services::description.ilike(&q))
            .order(schema::services::price.asc())
            .limit(limit) 
            .offset(offset)
            .load::<Service>(&_connection)
            .expect("E.");
    }
    pub fn count(organization_id: i32) -> usize {
        use crate::schema::services::dsl::services;

        let _connection = establish_connection();
        return services
            .filter(schema::services::organization_id.eq(organization_id))
            .select(schema::services::id)
            .load::<i32>(&_connection)
            .expect("E.")
            .len();
    }

    pub fn get_all() -> Vec<Service> {
        use crate::schema::services::dsl::services;

        let _connection = establish_connection();
        return services
            .load::<Service>(&_connection)
            .expect("E.");
    }
}