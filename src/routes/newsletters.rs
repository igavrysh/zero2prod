use actix_web::{HttpResponse, web};

#[derive(serde::Deserialize)]
pub struct BodyData {
    title: String,
    content: Content,
}

#[derive(serde::Deserialize)]
pub struct Content {
    html: String,
    text: String,
}

pub async fn publish_newsletter(body: web::Json<BodyData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}