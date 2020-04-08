use crate::user::User;

use actix_web::{web, HttpResponse, Responder};
// use serde_json::json;

async fn find_all() -> impl Responder {
    HttpResponse::Ok()
        .json(
            vec!(
                User::new("john.appleseed@email.com".to_string()),
                User::new("jane.doe@email.com".to_string())
            )
        )
}

async fn find() -> impl Responder {
    HttpResponse::Ok()
        .json(
            User::new("john.appleseed@email.com".to_string())
        )
}

pub fn endpoints(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").route(web::get().to(find_all)));
    cfg.service(web::resource("/{id}").route(web::get().to(find)));
}