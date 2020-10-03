use std::collections::HashMap;

use actix_web::HttpResponse;

use super::Backend;
use uuid::Uuid;

/// MemoryBackend provides an all in memory storage backend for the registry, this is useful only
/// for testing and should never be used in practice.
#[derive(Default, Clone)]
pub struct MemoryBackend {
    repos: HashMap<String, Repository>,
}

#[derive(Default, Clone)]
struct Repository {
    uploads: HashMap<String, Upload>,
    layers: HashMap<String, Layer>,
}

#[derive(Default, Clone)]
struct Upload(Vec<u8>);

#[derive(Default, Clone)]
struct Layer(Vec<u8>);

#[async_trait]
impl Backend for MemoryBackend {
    async fn start_upload(&mut self, repository: String) -> HttpResponse {
        let repo = self
            .repos
            .entry(repository.clone())
            .or_insert(Repository::default());

        let id = Uuid::new_v4();
        repo.uploads
            .insert(id.to_string(), Upload(Vec::with_capacity(0)));
        HttpResponse::Accepted()
            .header(
                "Location",
                format!("/v2/{}/blobs/uploads/{}", repository, id.to_string()),
            )
            .header("Range", "bytes=0-0")
            .header("Content-Length", "0")
            .header("Docker-Upload-UUID", id.to_string())
            .finish()
    }
}
