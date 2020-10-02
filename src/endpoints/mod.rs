use actix_web::web::Json;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

pub async fn v2() -> impl Responder {
    HttpResponse::Ok()
}

pub async fn upload(path: web::Path<(String, String)>) -> impl Responder {
    let unwrapped_path = path.into_inner();
    let mut body_data = unwrapped_path.0;
    body_data.push_str(&unwrapped_path.1);

    HttpResponse::Ok().body(&body_data)
}

pub async fn default_endpoint(req: HttpRequest) -> impl Responder {
    println!("{:?}", req.path());

    HttpResponse::NotFound()
}
