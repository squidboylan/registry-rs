use actix_web::HttpResponse;

pub mod memory;

#[async_trait]
pub trait Backend {
    async fn start_upload(&mut self, respository: String) -> HttpResponse;
}
