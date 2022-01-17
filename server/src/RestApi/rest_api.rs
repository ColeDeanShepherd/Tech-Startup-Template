use actix_web::{get, post, put, delete, web, App, HttpResponse, HttpServer, Responder};

#[get("/users/{user_id}")]
async fn echo() -> impl Responder {
    HttpResponse::Ok().body()
}

#[post("/users")]
async fn echo() -> impl Responder {
    HttpResponse::Ok().body()
}

#[delete("/users/{user_id}")]
async fn echo() -> impl Responder {
    HttpResponse::Ok().body()
}

#[get("/messages")]
async fn echo() -> impl Responder {
    HttpResponse::Ok().body()
}

#[post("/messages")]
async fn echo() -> impl Responder {
    HttpResponse::Ok().body()
}

#[delete("/messages/{message_id}")]
async fn echo() -> impl Responder {
    HttpResponse::Ok().body()
}

#[post("/videos")]
async fn echo() -> impl Responder {
    HttpResponse::Ok().body()
}

#[delete("/videos/{video_id}")]
async fn echo() -> impl Responder {
    HttpResponse::Ok().body()
}

#[post("/comments")]
async fn echo() -> impl Responder {
    HttpResponse::Ok().body()
}
