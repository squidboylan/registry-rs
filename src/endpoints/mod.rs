use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::storage::Backend;
use std::sync::Arc;

pub async fn v2() -> impl Responder {
    HttpResponse::Ok()
}

pub async fn upload(
    storage: web::Data<Arc<dyn Backend + Send + Sync>>,
    path: web::Path<(String, String)>,
) -> impl Responder {
    let unwrapped_path = path.into_inner();
    let mut repo_name = unwrapped_path.0;
    repo_name.push('/');
    repo_name.push_str(&unwrapped_path.1);

    storage.into_inner().start_upload(repo_name).await
}

pub async fn default_endpoint(req: HttpRequest) -> impl Responder {
    println!("{:?}", req.path());

    HttpResponse::NotFound()
}
