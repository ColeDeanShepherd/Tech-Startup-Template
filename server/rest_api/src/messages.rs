use actix_web::{get, post, delete, HttpResponse, Responder};

#[get("/messages")]
async fn get_messages() -> impl Responder {
    HttpResponse::Ok()
}

#[post("/messages")]
async fn send_message() -> impl Responder {
    HttpResponse::Ok()
}

#[delete("/messages/{message_id}")]
async fn delete_message() -> impl Responder {
    HttpResponse::Ok()
}