#[macro_use]
extern crate serde;

#[macro_use]
extern crate async_trait;

use actix_web::{web, App, HttpServer};

use std::sync::Arc;

mod endpoints;
mod storage;

use storage::Backend;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let backend: Arc<dyn Backend + Send + Sync> =
        Arc::new(storage::memory::MemoryBackend::default());
    HttpServer::new(move || {
        App::new()
            .data(backend.clone())
            .route("/v2", web::get().to(endpoints::v2))
            .route(
                "/v2/{namespace}/{name}/blobs/uploads/",
                web::post().to(endpoints::start_upload),
            )
            .route(
                "/v2/{namespace}/{name}/blobs/uploads/{id}",
                web::get().to(endpoints::get_upload),
            )
            .route(
                "/v2/{namespace}/{name}/blobs/uploads/{id}",
                web::put().to(endpoints::complete_upload),
            )
            .route(
                "/v2/{namespace}/{name}/blobs/uploads/{id}",
                web::delete().to(endpoints::delete_upload),
            )
            .default_service(web::to(endpoints::default_endpoint))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
