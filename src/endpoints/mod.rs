use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::storage::Backend;
use std::sync::Arc;
use tokio::sync::RwLock;

pub async fn v2() -> impl Responder {
    HttpResponse::Ok()
}

pub async fn upload(
    storage: web::Data<Arc<RwLock<crate::storage::memory::MemoryBackend>>>,
    path: web::Path<(String, String)>,
) -> impl Responder {
    let unwrapped_path = path.into_inner();
    let mut repo_name = unwrapped_path.0;
    repo_name.push('/');
    repo_name.push_str(&unwrapped_path.1);

    storage
        .into_inner()
        .write()
        .await
        .start_upload(repo_name)
        .await
}

pub async fn default_endpoint(req: HttpRequest) -> impl Responder {
    println!("{:?}", req.path());

    HttpResponse::NotFound()
}
