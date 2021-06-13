use actix_web::{ HttpResponse, Responder, web};

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}

#[actix_web::post("/subscriptions")]
pub async fn subscriptions(form: web::Form<FormData>) -> impl Responder {
    HttpResponse::Ok()
}
