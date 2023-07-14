use actix_web::{web, get, HttpResponse, Responder};

use crate::models::users_model::User;

#[get("/")]
pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

pub async fn get_all() -> impl Responder {
    let resulets = User::get_all();
    let res = format!("Displaying {} posts", resulets);

    HttpResponse::Ok().body(resulets)
}

pub async fn get_one(path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    let _resulets = User::get_one(id);

    HttpResponse::Ok().body(_resulets)
}

pub async fn post(info: web::Json<User>) -> impl Responder {
    User::post(info);

    HttpResponse::Created().body("post OK.")
}

pub async fn update(info: web:: Json<User>) -> impl Responder {
    User:: update(info);

    HttpResponse::Ok().body("update ok.")
}

pub async fn delete(path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    User:: delete(id);

    HttpResponse::Ok().body("delete ok.")
}

