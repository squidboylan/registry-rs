use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::storage::Backend;
use actix_web::web::Query;
use bytes::Bytes;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct Digest {
    digest: String,
}

pub async fn v2() -> impl Responder {
    HttpResponse::Ok()
}

pub async fn start_upload(
    storage: web::Data<Arc<dyn Backend + Send + Sync>>,
    path: web::Path<(String, String)>,
) -> impl Responder {
    let unwrapped_path = path.into_inner();
    let mut repo_name = unwrapped_path.0;
    repo_name.push('/');
    repo_name.push_str(&unwrapped_path.1);

    storage.into_inner().start_upload(repo_name).await
}

pub async fn get_upload(
    storage: web::Data<Arc<dyn Backend + Send + Sync>>,
    path: web::Path<(String, String, String)>,
) -> impl Responder {
    let unwrapped_path = path.into_inner();
    let mut repo_name = unwrapped_path.0;
    repo_name.push('/');
    repo_name.push_str(&unwrapped_path.1);
    let id = unwrapped_path.2;

    storage.into_inner().get_upload(repo_name, id).await
}

pub async fn complete_upload(
    req: HttpRequest,
    storage: web::Data<Arc<dyn Backend + Send + Sync>>,
    path: web::Path<(String, String, String)>,
    data: Bytes,
    digest: Query<Digest>,
) -> impl Responder {
    let unwrapped_path = path.into_inner();
    let mut repo_name = unwrapped_path.0;
    repo_name.push('/');
    repo_name.push_str(&unwrapped_path.1);
    let id = unwrapped_path.2;
    let tmp = req.headers().get("Content-Range");
    let range = if let Some(x) = tmp {
        Some(x.to_str().unwrap().to_owned())
    } else {
        None
    };

    storage
        .into_inner()
        .complete_upload(repo_name, id, &data, digest.digest.clone(), range)
        .await
}

pub async fn chunk_upload(
    req: HttpRequest,
    storage: web::Data<Arc<dyn Backend + Send + Sync>>,
    path: web::Path<(String, String, String)>,
    data: Bytes,
) -> impl Responder {
    let unwrapped_path = path.into_inner();
    let mut repo_name = unwrapped_path.0;
    repo_name.push('/');
    repo_name.push_str(&unwrapped_path.1);
    let id = unwrapped_path.2;

    let range = req
        .headers()
        .get("Content-Range")
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();

    storage
        .into_inner()
        .chunk_upload(repo_name, id, &data, range)
        .await
}

pub async fn delete_upload(
    storage: web::Data<Arc<dyn Backend + Send + Sync>>,
    path: web::Path<(String, String, String)>,
) -> impl Responder {
    let unwrapped_path = path.into_inner();
    let mut repo_name = unwrapped_path.0;
    repo_name.push('/');
    repo_name.push_str(&unwrapped_path.1);
    let id = unwrapped_path.2;

    storage.into_inner().delete_upload(repo_name, id).await
}

pub async fn head_layer(
    storage: web::Data<Arc<dyn Backend + Send + Sync>>,
    path: web::Path<(String, String, String)>,
) -> impl Responder {
    let unwrapped_path = path.into_inner();
    let mut repo_name = unwrapped_path.0;
    repo_name.push('/');
    repo_name.push_str(&unwrapped_path.1);
    let digest = unwrapped_path.2;

    storage.into_inner().head_layer(repo_name, digest).await
}

pub async fn default_endpoint(req: HttpRequest) -> impl Responder {
    println!("{:?}", req.path());

    HttpResponse::NotFound()
}
