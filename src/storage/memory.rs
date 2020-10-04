use std::collections::HashMap;
use std::sync::RwLock;

use actix_web::HttpResponse;
use bytes::Bytes;

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

    async fn get_upload(&self, repository: String, id: String) -> HttpResponse {
        let repos_lock = self.repos.read().unwrap();
        let repo = repos_lock.get(&repository).unwrap();

        let uploads_lock = repo.uploads.read().unwrap();
        let upload = uploads_lock.get(&id);

        match upload {
            Some(u) => HttpResponse::NoContent()
                .header(
                    "Location",
                    format!("/v2/{}/blobs/uploads/{}", repository, id),
                )
                .header("Range", format!("bytes=0-{}", u.0.len()))
                .header("Docker-Upload-UUID", id)
                .finish(),
            None => HttpResponse::NotFound().finish(),
        }
    }

    async fn complete_upload(
        &self,
        repository: String,
        id: String,
        data: &Bytes,
        digest: String,
        range: Option<String>,
    ) -> HttpResponse {
        let repos_lock = self.repos.read().unwrap();
        let repo = repos_lock.get(&repository);
        if repo.is_none() {
            return HttpResponse::NotFound().finish();
        }
        let repo = repo.unwrap();

        let mut range_vec: Option<Vec<usize>> = None;
        let upload = {
            let mut uploads_lock = repo.uploads.write().unwrap();
            if range.is_some() {
                let upload = uploads_lock.get(&id);
                if upload.is_none() {
                    return HttpResponse::NotFound().finish();
                }
                let upload = upload.unwrap();
                let tmp: Vec<usize> = range
                    .unwrap()
                    .split("-")
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect();
                if upload.0.len() != tmp[0] {
                    return HttpResponse::RangeNotSatisfiable()
                        .header(
                            "Location",
                            format!("/v2/{}/blobs/uploads/{}", repository, id),
                        )
                        .header("Range", format!("0-{}", upload.0.len() - 1))
                        .header("Content-Length", "0")
                        .header("Docker-Upload-UUID", id)
                        .finish();
                }
                range_vec = Some(tmp);
            }
            uploads_lock.remove(&id)
        };

        if upload.is_none() {
            return HttpResponse::NotFound().finish();
        }
        let mut upload_vec = upload.unwrap().0;
        upload_vec.extend_from_slice(data);
        let len = upload_vec.len();
        {
            repo.layers
                .write()
                .unwrap()
                .insert(digest.clone(), Layer(upload_vec));
        }

        if range_vec.is_some() {
            return HttpResponse::Created()
                .header("Location", format!("/v2/{}/blobs/{}", repository, digest))
                .header("Content-Length", "0")
                .header("Docker-Content-Digest", digest)
                .header("Range", format!("0-{}", len))
                .finish();
        } else {
            return HttpResponse::Created()
                .header("Location", format!("/v2/{}/blobs/{}", repository, digest))
                .header("Content-Length", "0")
                .header("Docker-Content-Digest", digest)
                .finish();
        }
    }

    async fn chunk_upload(
        &self,
        repository: String,
        id: String,
        data: &Bytes,
        range: String,
    ) -> HttpResponse {
        let range_vec: Vec<usize> = range
            .split("-")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        let repos_lock = self.repos.read().unwrap();
        let repo = repos_lock.get(&repository);
        if repo.is_none() {
            return HttpResponse::NotFound().finish();
        }
        let repo = repo.unwrap();

        let mut uploads_lock = repo.uploads.write().unwrap();
        let upload = uploads_lock.remove(&id);

        if upload.is_none() {
            return HttpResponse::NotFound().finish();
        }
        let mut upload_vec = upload.unwrap().0;
        if upload_vec.len() != range_vec[0] {
            return HttpResponse::RangeNotSatisfiable()
                .header(
                    "Location",
                    format!("/v2/{}/blobs/uploads/{}", repository, id),
                )
                .header("Range", format!("0-{}", upload_vec.len() - 1))
                .header("Content-Length", "0")
                .header("Docker-Upload-UUID", id)
                .finish();
        }
        upload_vec.extend_from_slice(data);

        HttpResponse::Created()
            .header(
                "Location",
                format!("/v2/{}/blobs/uploads/{}", repository, id),
            )
            .header("Range", format!("0-{}", upload_vec.len()))
            .header("Content-Length", "0")
            .header("Docker-Upload-UUID", id)
            .finish()
    }

    async fn delete_upload(&self, repository: String, id: String) -> HttpResponse {
        let repos_lock = self.repos.read().unwrap();
        let repo = repos_lock.get(&repository).unwrap();

        let mut uploads_lock = repo.uploads.write().unwrap();
        let upload = uploads_lock.remove(&id);

        match upload {
            Some(u) => HttpResponse::Ok()
                .header(
                    "Location",
                    format!("/v2/{}/blobs/uploads/{}", repository, id),
                )
                .header("Range", format!("bytes=0-{}", u.0.len()))
                .header("Docker-Upload-UUID", id)
                .finish(),
            None => HttpResponse::NotFound().finish(),
        }
    }

    async fn head_layer(&self, repository: String, digest: String) -> HttpResponse {
        let repos_lock = self.repos.read().unwrap();
        let repo = repos_lock.get(&repository).unwrap();

        let layers_lock = repo.layers.read().unwrap();
        let layer = layers_lock.get(&digest);

        match layer {
            Some(Layer(l)) => HttpResponse::Ok()
                .header("Content-Length", l.len())
                .header("Docker-Content-Digest", digest)
                .finish(),
            None => HttpResponse::NotFound().finish(),
        }
    }
}
