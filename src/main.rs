#[macro_use]
extern crate serde_derive;

use actix_web::web::Json;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

mod backends;
mod endpoints;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .route("/v2", web::get().to(endpoints::v2))
            .route(
                "/v2/{namespace}/{name}/blobs/uploads/",
                web::get().to(endpoints::upload),
            )
            .default_service(web::to(endpoints::default_endpoint))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
