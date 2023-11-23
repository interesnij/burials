// @generated automatically by Diesel CLI.

diesel::table! {
    cities (id) {
        id -> Int4,
        name -> Varchar,
        geo_id -> Nullable<Int4>,
        region_id -> Nullable<Int4>,
        country_id -> Int4,
        lat -> Nullable<Float8>,
        lon -> Nullable<Float8>,
    }
}

diesel::table! {
    countries (id) {
        id -> Int4,
        name -> Varchar,
        geo_id -> Nullable<Int4>,
        continent_id -> Nullable<Int4>,
        timezone_id -> Nullable<Int4>,
        phone -> Nullable<Varchar>,
        lat -> Nullable<Float8>,
        lon -> Nullable<Float8>,
    }
}

diesel::table! {
    deceaseds (id) {
        id -> Int4,
        user_id -> Int4,
        place_id -> Int4,
        first_name -> Varchar,
        middle_name -> Nullable<Varchar>,
        last_name -> Varchar,
        birth_date -> Date,
        death_date -> Date,
        image -> Nullable<Varchar>,
        memory_words -> Nullable<Varchar>,
        lat -> Float8,
        lon -> Float8,
    }
}

diesel::table! {
    districts (id) {
        id -> Int4,
        name -> Varchar,
        region_id -> Nullable<Int4>,
        country_id -> Int4,
        lat -> Nullable<Float8>,
        lon -> Nullable<Float8>,
    }
}

diesel::table! {
    organizations (id) {
        id -> Int4,
        name -> Varchar,
        description -> Varchar,
        director -> Varchar,
        phone -> Varchar,
        hours -> Varchar,
        website -> Nullable<Varchar>,
        image -> Nullable<Varchar>,
        user_id -> Int4,
    }
}

diesel::table! {
    organizations_places (id) {
        id -> Int4,
        organization_id -> Int4,
        city_id -> Nullable<Int4>,
        district_id -> Nullable<Int4>,
        region_id -> Nullable<Int4>,
        country_id -> Int4,
        lat -> Nullable<Float8>,
        lon -> Nullable<Float8>,
    }
}

diesel::table! {
    places (id) {
        id -> Int4,
        user_id -> Int4,
        city_id -> Nullable<Int4>,
        district_id -> Nullable<Int4>,
        region_id -> Nullable<Int4>,
        country_id -> Int4,
        title -> Varchar,
        description -> Nullable<Varchar>,
        hours -> Nullable<Varchar>,
        image -> Nullable<Varchar>,
        address -> Nullable<Varchar>,
        count -> Int2,
        director -> Nullable<Varchar>,
        phone -> Nullable<Varchar>,
        lat -> Float8,
        lon -> Float8,
    }
}

diesel::table! {
    regions (id) {
        id -> Int4,
        name -> Varchar,
        geo_id -> Nullable<Int4>,
        country_id -> Int4,
        timezone_id -> Nullable<Int4>,
        lat -> Nullable<Float8>,
        lon -> Nullable<Float8>,
    }
}

diesel::table! {
    reviews (id) {
        id -> Int4,
        service_id -> Int4,
        user_id -> Int4,
        content -> Varchar,
        created -> Timestamp,
    }
}

diesel::table! {
    services (id) {
        id -> Int4,
        user_id -> Int4,
        organization_id -> Int4,
        city_id -> Int4,
        title -> Varchar,
        description -> Varchar,
        image -> Nullable<Varchar>,
        price -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        description -> Nullable<Text>,
        image -> Nullable<Varchar>,
        perm -> Int2,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    cities,
    countries,
    deceaseds,
    districts,
    organizations,
    organizations_places,
    places,
    regions,
    reviews,
    services,
    users,
);
