// @generated automatically by Diesel CLI.

diesel::table! {
    products (id) {
        id -> Integer,
        name -> Text,
        description -> Text,
        price -> Double,
        category_id -> Integer,
    }
}
