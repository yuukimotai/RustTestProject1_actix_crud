use actix_web::{web};

use crate::controllers::users_controller;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/users", web::get().to(users_controller::get_all))
        .route("/users/{id}", web::get().to(users_controller::get_one))
        .route("/users", web::post().to(users_controller::post))
        .route("/users", web::put().to(users_controller::update))
        .route("/users/{id}", web::delete().to(users_controller::delete));
}