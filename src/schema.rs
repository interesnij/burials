// @generated automatically by Diesel CLI.

diesel::table! {
    cities (id) {
        id -> Int4,
        #[max_length = 100]
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
        #[max_length = 100]
        name -> Varchar,
        geo_id -> Nullable<Int4>,
        continent_id -> Nullable<Int4>,
        timezone_id -> Nullable<Int4>,
        #[max_length = 10]
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
        #[max_length = 100]
        first_name -> Varchar,
        #[max_length = 100]
        middle_name -> Nullable<Varchar>,
        #[max_length = 100]
        last_name -> Varchar,
        birth_date -> Date,
        death_date -> Date,
        #[max_length = 100]
        image -> Nullable<Varchar>,
        #[max_length = 500]
        memory_words -> Nullable<Varchar>,
        lat -> Float8,
        lon -> Float8,
        types -> Int4,
    }
}

diesel::table! {
    districts (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        region_id -> Nullable<Int4>,
        country_id -> Int4,
        lat -> Nullable<Float8>,
        lon -> Nullable<Float8>,
    }
}

diesel::table! {
    files (id) {
        id -> Int4,
        object_id -> Int4,
        object_types -> Int2,
        #[max_length = 100]
        src -> Varchar,
    }
}

diesel::table! {
    organizations (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 500]
        description -> Varchar,
        #[max_length = 255]
        director -> Varchar,
        #[max_length = 15]
        phone -> Varchar,
        #[max_length = 100]
        hours -> Varchar,
        #[max_length = 100]
        website -> Nullable<Varchar>,
        #[max_length = 100]
        image -> Nullable<Varchar>,
        user_id -> Int4,
        types -> Int4,
    }
}

diesel::table! {
    organizations_places (id) {
        id -> Int4,
        organization_id -> Int4,
        city_id -> Int4,
        region_id -> Nullable<Int4>,
        country_id -> Int4,
        #[max_length = 500]
        address2 -> Varchar,
    }
}

diesel::table! {
    organizations_services (id) {
        id -> Int4,
        organization_id -> Int4,
        service_id -> Int4,
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
        #[max_length = 100]
        title -> Varchar,
        #[max_length = 500]
        description -> Nullable<Varchar>,
        #[max_length = 100]
        hours -> Nullable<Varchar>,
        #[max_length = 100]
        image -> Nullable<Varchar>,
        #[max_length = 255]
        address -> Nullable<Varchar>,
        count -> Int2,
        #[max_length = 255]
        director -> Nullable<Varchar>,
        #[max_length = 15]
        phone -> Nullable<Varchar>,
        lat -> Float8,
        lon -> Float8,
        types -> Int4,
    }
}

diesel::table! {
    regions (id) {
        id -> Int4,
        #[max_length = 100]
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
        #[max_length = 1000]
        content -> Varchar,
        created -> Timestamp,
    }
}

diesel::table! {
    services (id) {
        id -> Int4,
        #[max_length = 100]
        title -> Varchar,
        position -> Int2,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 100]
        username -> Varchar,
        #[max_length = 100]
        first_name -> Varchar,
        #[max_length = 100]
        last_name -> Varchar,
        #[max_length = 12]
        phone -> Varchar,
        #[max_length = 100]
        email -> Varchar,
        #[max_length = 100]
        password -> Varchar,
        description -> Nullable<Text>,
        #[max_length = 100]
        image -> Nullable<Varchar>,
        perm -> Int2,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    cities,
    countries,
    deceaseds,
    districts,
    files,
    organizations,
    organizations_places,
    organizations_services,
    places,
    regions,
    reviews,
    services,
    users,
);
