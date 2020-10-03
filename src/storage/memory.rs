use std::collections::HashMap;
use std::sync::RwLock;

use actix_web::HttpResponse;

use super::Backend;
use uuid::Uuid;

/// MemoryBackend provides an all in memory storage backend for the registry, this is useful only
/// for testing and should never be used in practice.
#[derive(Default)]
pub struct MemoryBackend {
    repos: RwLock<HashMap<String, Repository>>,
}

#[derive(Default)]
struct Repository {
    uploads: RwLock<HashMap<String, Upload>>,
    layers: RwLock<HashMap<String, Layer>>,
}

#[derive(Default, Clone)]
struct Upload(Vec<u8>);

#[derive(Default, Clone)]
struct Layer(Vec<u8>);

#[async_trait]
impl Backend for MemoryBackend {
    async fn start_upload(&self, repository: String) -> HttpResponse {
        let mut repos_lock = self.repos.write().unwrap();
        let repo = repos_lock
            .entry(repository.clone())
            .or_insert(Repository::default());

        let id = Uuid::new_v4();
        repo.uploads
            .write()
            .unwrap()
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
