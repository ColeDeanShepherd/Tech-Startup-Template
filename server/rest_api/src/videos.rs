use actix_web::{post, delete, HttpResponse, Responder};

#[post("/videos")]
async fn upload_video() -> impl Responder {
    HttpResponse::Ok()
}

#[delete("/videos/{video_id}")]
async fn delete_video() -> impl Responder {
    HttpResponse::Ok()
}