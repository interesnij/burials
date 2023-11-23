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
use chrono::NaiveDateTime;
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
    pub created:    NaiveDateTime,
}

// Структура для создания нового отзыва
#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "reviews"]
pub struct NewReview {
    pub service_id: i32,
    pub user_id:    i32,
    pub content:    String,
    pub created:    NaiveDateTime,
}

impl Review {
    pub fn create (
        user_id:    i32,
        service_id: i32,
        content:    String,
    ) -> i16 {
        let _connection = establish_connection();
        let _user = crate::utils::get_user(user_id).expect("E.");
        if _user.perm > 10 {
            let new_form = NewReview {
                service_id: service_id,
                user_id:    user_id,
                content:    content,
                created:    chrono::Local::now().naive_utc(),
            };
            diesel::insert_into(schema::reviews::table)
                .values(&new_form)
                .execute(&_connection)
                .expect("Error.");
        }
        return 1;
    }
    pub fn edit (
        user_id:   i32,
        object_id: i32,
        content:   String,
    ) -> i16 {
        use crate::schema::reviews::dsl::reviews;

        let _connection = establish_connection();
        let _review = crate::utils::get_review(object_id).expect("E.");
        let _user = crate::utils::get_user(user_id).expect("E.");
        if _user.perm > 10 || _review.user_id == user_id {
            diesel::update(&_review)
                .set(schema::reviews::content.eq(content))
                .execute(&_connection)
                .expect("Error.");
        }
        return 1;
    }
    pub fn delete(&self) -> i16 {
        use crate::schema::reviews::dsl::reviews;

        let _connection = establish_connection();
        diesel::delete(reviews.filter(schema::reviews::id.eq(self.id)))
            .execute(&_connection)
            .expect("E");
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