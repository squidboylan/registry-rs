use actix_web::HttpResponse;
use bytes::Bytes;

pub mod memory;

#[async_trait]
pub trait Backend {
    async fn start_upload(&self, repository: String) -> HttpResponse;
    async fn get_upload(&self, repository: String, id: String) -> HttpResponse;
    async fn complete_upload(
        &self,
        repository: String,
        id: String,
        data: &Bytes,
        digest: Option<String>,
        range: Option<String>,
    ) -> HttpResponse;
    async fn delete_upload(&self, repository: String, id: String) -> HttpResponse;
    async fn head_layer(&self, repository: String, digest: String) -> HttpResponse;
}
