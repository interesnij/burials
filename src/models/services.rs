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
    pub id: i32,
    pub user_id:         i32,
    pub organization_id: i32,
    pub city_id:         i32,
    pub title:           String,
    pub description:     String,
    pub image:           Option<String>,
    pub price:           i32,
}

// Структура NewService используется для создания новых объектов Service
#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "services"]
pub struct NewService {
    pub user_id:         i32,
    pub organization_id: i32,
    pub city_id:         i32,
    pub title:           String,
    pub description:     String,
    pub image:           Option<String>,
    pub price:           i32,
}

impl Service {
    pub fn create (
        user_id:         i32,
        organization_id: i32,
        city_id:         i32,
        title:           String,
        description:     String,
        image:           Option<String>,
        price:           i32,
    ) -> i16 {
        let _connection = establish_connection();
        let _user = crate::utils::get_user(user_id).expect("E.");
        let _organization = crate::utils::get_organization(organization_id).expect("E.");

        if _organization.user_id == user_id || _user.perm > 10 {
            let new_form = NewService {
                user_id:         user_id,
                organization_id: organization_id,
                city_id:         city_id,
                title:           title,
                description:     description,
                image:           image,
                price:           price,
            };
            diesel::insert_into(schema::services::table)
                .values(&new_form)
                .execute(&_connection)
                .expect("Error.");
        }
        return 1;
    }
    pub fn edit (
        user_id:     i32,
        object_id:   i32,
        city_id:     i32,
        title:       String,
        description: String,
        image:       Option<String>,
        price:       i32,
    ) -> i16 {
        use crate::schema::services::dsl::services;

        let _connection = establish_connection();
        let _service = crate::utils::get_service(object_id).expect("E.");
        let _user = crate::utils::get_user(user_id).expect("E.");
        let _organization = crate::utils::get_organization(_service.organization_id).expect("E.");
        if _user.perm > 10 || _organization.user_id == user_id {
            diesel::update(&_service)
                .set((
                    schema::services::city_id.eq(city_id),
                    schema::services::title.eq(title),
                    schema::services::description.eq(description),
                    schema::services::image.eq(image),
                    schema::services::price.eq(price),
                ))
                .execute(&_connection)
                .expect("Error.");
        }
        return 1;
    }
    pub fn delete(&self) -> i16 {
        use crate::schema::services::dsl::services;

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
}