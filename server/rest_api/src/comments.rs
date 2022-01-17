use actix_web::{post, HttpResponse, Responder};

#[post("/comments")]
async fn post_comment() -> impl Responder {
    HttpResponse::Ok()
}