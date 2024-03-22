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
use crate::schema::reviews;
use serde::{Serialize, Deserialize};


#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct Review {
    pub id:         i32,
    pub service_id: i32,
    pub user_id:    i32,
    pub content:    String,
    pub created:    chrono::NaiveDateTime,
}

// Структура для создания нового отзыва
#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "reviews"]
pub struct NewReview {
    pub service_id: i32,
    pub user_id:    i32,
    pub content:    String,
    pub created:    chrono::NaiveDateTime,
}

impl Review {
    pub fn create (
        user_id:    i32,
        service_id: i32,
        content:    String,
    ) -> i16 { 
        let _connection = establish_connection();
        let _user = crate::utils::get_user(user_id).expect("E.");
        if _user.perm > 9 {
            let new_form = NewReview {
                service_id: service_id,
                user_id:    user_id,
                content:    content, 
                created:    chrono::Local::now().naive_utc(),
            };
            let _new = diesel::insert_into(schema::reviews::table)
                .values(&new_form)
                .get_result::<Review>(&_connection)
                .expect("Error.");
            crate::models::Log::create(user_id, _new.id, 5, 1);
        }
        return 1;
    }
    pub fn edit (
        &self,
        user_id: i32,
        content: String,
    ) -> i16 {
        use crate::schema::reviews::dsl::reviews;

        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::reviews::content.eq(content))
            .execute(&_connection)
            .expect("Error.");
        crate::models::Log::create(user_id, self.id, 5, 2);
        
        return 1;
    }
    pub fn delete(&self, user_id: i32) -> i16 {
        use crate::schema::reviews::dsl::reviews;

        let _connection = establish_connection();
        diesel::delete(reviews.filter(schema::reviews::id.eq(self.id)))
            .execute(&_connection)
            .expect("E");
        crate::models::Log::create(user_id, self.id, 5, 3);
        return 1;
    }

    pub fn list (
        service_id: i32,
        limit:      i64,
        offset:     i64,
    ) -> Vec<Review> {
        use crate::schema::reviews::dsl::reviews;

        let _connection = establish_connection();
        return reviews
            .filter(schema::reviews::service_id.eq(service_id))
            .order(schema::reviews::created.desc())
            .limit(limit)
            .offset(offset)
            .load::<Review>(&_connection)
            .expect("E.");
    }
    pub fn search (
        service_id: i32,
        q:          &String,
        limit:      i64,
        offset:     i64,
    ) -> Vec<Review> {
        use crate::schema::reviews::dsl::reviews;

        let _connection = establish_connection();
        return reviews
            .filter(schema::reviews::service_id.eq(service_id))
            .filter(schema::reviews::content.ilike(&q))
            .order(schema::reviews::created.desc())
            .limit(limit)
            .offset(offset)
            .load::<Review>(&_connection)
            .expect("E.");
    }
    pub fn count(service_id: i32) -> usize {
        use crate::schema::reviews::dsl::reviews;

        let _connection = establish_connection();
        return reviews
            .filter(schema::reviews::service_id.eq(service_id))
            .select(schema::reviews::id)
            .load::<i32>(&_connection)
            .expect("E.")
            .len();
    }
}