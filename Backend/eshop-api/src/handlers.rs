use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use crate::models::{NewUser, NewCategory, NewProduct, NewOrder, User, Category, Product, Order};
use crate::schema::{users, categories, products, orders};
use crate::AppState;
use bcrypt::{hash, verify};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginCredentials {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

// Implement user registration
pub async fn register(
    new_user: web::Json<NewUser>,
    state: web::Data<AppState>,
) -> impl Responder {
    let conn = &mut state.pool.get().unwrap();
    let hashed_password = hash(&new_user.password, 4).unwrap();
    let user = NewUser {
        username: new_user.username.clone(),
        password: hashed_password,
    };

    match diesel::insert_into(users::table)
        .values(&user)
        .execute(conn)
    {
        Ok(_) => HttpResponse::Created().json("User created successfully"),
        Err(_) => HttpResponse::InternalServerError().json("Error creating user"),
    }
}

// Implement user login
pub async fn login(
    credentials: web::Json<LoginCredentials>,
    state: web::Data<AppState>,
) -> impl Responder {
    let conn = &mut state.pool.get().unwrap();
    let user = users::table
        .filter(users::username.eq(&credentials.username))
        .first::<User>(conn)
        .optional();

    match user {
        Ok(Some(user)) => {
            if verify(&credentials.password, &user.password_hash).unwrap() {
                let claims = Claims {
                    sub: user.username,
                    exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
                };
                let token = encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref())).unwrap();
                HttpResponse::Ok().json(token)
            } else {
                HttpResponse::Unauthorized().json("Invalid credentials")
            }
        }
        _ => HttpResponse::Unauthorized().json("Invalid credentials"),
    }
}

// Implement product creation (admin only)
pub async fn create_product(
    new_product: web::Json<NewProduct>,
    state: web::Data<AppState>,
) -> impl Responder {
    let conn = &mut state.pool.get().unwrap();
    match diesel::insert_into(products::table)
        .values(&*new_product)
        .execute(conn)
    {
        Ok(_) => HttpResponse::Created().json("Product created successfully"),
        Err(_) => HttpResponse::InternalServerError().json("Error creating product"),
    }
}

// Implement get all products
pub async fn get_products(state: web::Data<AppState>) -> impl Responder {
    let conn = &mut state.pool.get().unwrap();
    match products::table.load::<Product>(conn) {
        Ok(products) => HttpResponse::Ok().json(products),
        Err(_) => HttpResponse::InternalServerError().json("Error fetching products"),
    }
}

// Implement category creation (admin only)
pub async fn create_category(
    new_category: web::Json<NewCategory>,
    state: web::Data<AppState>,
) -> impl Responder {
    let conn = &mut state.pool.get().unwrap();
    match diesel::insert_into(categories::table)
        .values(&*new_category)
        .execute(conn)
    {
        Ok(_) => HttpResponse::Created().json("Category created successfully"),
        Err(_) => HttpResponse::InternalServerError().json("Error creating category"),
    }
}

// Implement get all categories
pub async fn get_categories(state: web::Data<AppState>) -> impl Responder {
    let conn = &mut state.pool.get().unwrap();
    match categories::table.load::<Category>(conn) {
        Ok(categories) => HttpResponse::Ok().json(categories),
        Err(_) => HttpResponse::InternalServerError().json("Error fetching categories"),
    }
}

// Implement order creation (customer only)
pub async fn create_order(
    new_order: web::Json<NewOrder>,
    state: web::Data<AppState>,
) -> impl Responder {
    let conn = &mut state.pool.get().unwrap();
    match diesel::insert_into(orders::table)
        .values(&*new_order)
        .execute(conn)
    {
        Ok(_) => HttpResponse::Created().json("Order created successfully"),
        Err(_) => HttpResponse::InternalServerError().json("Error creating order"),
    }
}

// Implement get user orders
pub async fn get_user_orders(user_id: web::Path<i32>, state: web::Data<AppState>) -> impl Responder {
    let conn = &mut state.pool.get().unwrap();
    match orders::table
        .filter(orders::user_id.eq(user_id.into_inner()))
        .load::<Order>(conn)
    {
        Ok(orders) => HttpResponse::Ok().json(orders),
        Err(_) => HttpResponse::InternalServerError().json("Error fetching orders"),
    }
}