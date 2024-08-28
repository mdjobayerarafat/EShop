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
diesel::joinable!(products -> categories (category_id));
diesel::joinable!(orders -> users (user_id));
diesel::joinable!(orders -> products (product_id));

diesel::allow_tables_to_appear_in_same_query!(
    users,
    categories,
    products,
    orders,
);