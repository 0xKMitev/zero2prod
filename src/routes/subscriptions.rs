use actix_web::{web, HttpResponse};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subscribe(form: web::Form<FormData>) -> HttpResponse {
    if form.name.is_empty() || form.email.is_empty() {
        return HttpResponse::BadRequest().finish();
    }
    HttpResponse::Ok().finish()
}
