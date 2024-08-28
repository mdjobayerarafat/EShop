// @generated automatically by Diesel CLI.
diesel::table! {
    users (id) {
        id -> Integer,
        username -> Text,
        password_hash -> Text,
        is_admin -> Bool,
    }
}

diesel::table! {
    categories (id) {
        id -> Integer,
        name -> Text,
    }
}


diesel::table! {
    products (id) {
        id -> Integer,
        name -> Text,
        description -> Text,
        price -> Double,
        category_id -> Integer,
    }
}

diesel::table! {
    orders (id) {
        id -> Integer,
        user_id -> Integer,
        product_id -> Integer,
        total_price -> Double,
        created_at -> Timestamp,
    }
}