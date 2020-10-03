#[macro_use]
extern crate async_trait;

use actix_web::{web, App, HttpServer};

use std::sync::Arc;
use tokio::sync::RwLock;

mod endpoints;
mod storage;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let backend = Arc::new(RwLock::new(storage::memory::MemoryBackend::default()));
    HttpServer::new(move || {
        App::new()
            .data(backend.clone())
            .route("/v2", web::get().to(endpoints::v2))
            .route(
                "/v2/{namespace}/{name}/blobs/uploads/",
                web::post().to(endpoints::upload),
            )
            .default_service(web::to(endpoints::default_endpoint))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
