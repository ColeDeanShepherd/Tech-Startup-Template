use actix_web::{get, post, delete, HttpResponse, Responder};

#[get("/users/{user_id}")]
async fn get_user() -> impl Responder {
    HttpResponse::Ok()
}

#[post("/users")]
async fn create_user() -> impl Responder {
    HttpResponse::Ok()
}

#[delete("/users/{user_id}")]
async fn delete_user() -> impl Responder {
    HttpResponse::Ok()
}