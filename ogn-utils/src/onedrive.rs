use std::ops::Range;
use std::path::Path;

use anyhow::anyhow;
use http::{Request, Response, StatusCode};
use hyper::body::Buf;
use hyper::client::HttpConnector;
use hyper::{Body, Client};
use serde::{Deserialize, Serialize};
use serde_derive::{Deserialize, Serialize};
use tokio::io::AsyncReadExt;

use crate::result::{Error, Result};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DriveId(pub String);

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserId(pub String);

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SiteId(pub String);

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UploadUrl(pub String);

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListDrivesRoot {
    pub value: Vec<ListDrivesValue>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListDrivesValue {
    pub id: DriveId,
    pub drive_type: String,
    pub name: String,
    pub owner: Owner,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Owner {
    pub user: User,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: UserId,
    pub display_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Drive {
    pub id: DriveId,
    pub drive_type: String,
    pub owner: Owner,
    pub quota: Quota,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quota {
    pub deleted: i64,
    pub file_count: i64,
    pub remaining: i64,
    pub state: String,
    pub total: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Site {
    pub id: SiteId,
    pub display_name: String,
    pub name: String,
    pub created_date_time: String,
    pub last_modified_date_time: String,
    pub web_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadSession {
    pub upload_url: UploadUrl,
    pub expiration_date_time: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadFileChunk {
    pub expiration_date_time: String,
    pub next_expected_ranges: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadFileFinished {
    pub id: String,
    pub name: String,
    pub size: i64,
    pub file: File,
}

pub enum UploadFile {
    Chunk(UploadFileChunk),
    Finished(UploadFileFinished),
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct File {}

#[derive(Default, Debug, Clone)]
pub struct Onedrive {
    client: Client<HttpConnector>,
    access_token: String,
}

impl Onedrive {
    const _SITE_NAME: &'static str = "organic_notebook"; // TODO: can't seem to create a site from the API
    const DRIVE_NAME_FOR_CONVERSION: &'static str = "document_conversion";

    pub fn new(token: &str) -> Self {
        Self {
            client: Client::builder().build_http(),
            access_token: token.to_string(),
        }
    }

    fn __default_cb(response: &Response<Body>, statuses: &[StatusCode]) -> Result<bool> {
        if statuses.contains(&response.status()) {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn default_cb(statuses: &[StatusCode]) -> impl Fn(&Response<Body>) -> Result<bool> + '_ {
        return |response: &Response<Body>| Self::__default_cb(response, statuses);
    }

    pub async fn request_raw<'de>(&self, method: &str, endpoint: &str) -> Result<Response<Body>> {
        const REQUEST_BASEURL: &'static str = "https://graph.microsoft.com/v1.0/";
        let request = Request::builder()
            .uri(format!("{}{}", REQUEST_BASEURL, endpoint))
            .header("Authorization", format!("Bearer {}", self.access_token))
            .method(method)
            .body(Body::empty())?;

        let response = self.client.request(request).await?;

        Ok(response)
    }

    pub async fn request_json_body<'de, TIn, TOut>(
        &self,
        method: &str,
        endpoint: &str,
        body: &TIn,
        cb: impl FnOnce(&Response<Body>) -> Result<bool>,
    ) -> Result<TOut>
    where
        TIn: Serialize,
        TOut: Deserialize<'de>,
    {
        const REQUEST_BASEURL: &'static str = "https://graph.microsoft.com/v1.0/me/";
        let request = Request::builder()
            .uri(format!("{}{}", REQUEST_BASEURL, endpoint))
            .header("Authorization", format!("Bearer {}", self.access_token))
            .header("Content-Type", "application/json")
            .method(method)
            .body(Body::from(serde_json::to_string(body)?))?;

        let response = self.client.request(request).await?;

        if !cb(&response)? {
            return Err(Error::new(anyhow!(
                "onedrive request failed with code: {}",
                response.status()
            )));
        }

        let body = response.into_body();
        let mut de =
            serde_json::Deserializer::from_reader(hyper::body::to_bytes(body).await?.reader());
        let response_body_json = TOut::deserialize(&mut de)?;

        Ok(response_body_json)
    }

    pub async fn request_empty<'de, TOut>(
        &self,
        method: &str,
        endpoint: &str,
        cb: impl FnOnce(&Response<Body>) -> Result<bool>,
    ) -> Result<TOut>
    where
        TOut: Deserialize<'de>,
    {
        const REQUEST_BASEURL: &'static str = "https://graph.microsoft.com/v1.0/";
        let request = Request::builder()
            .uri(format!("{}{}", REQUEST_BASEURL, endpoint))
            .header("Authorization", format!("Bearer {}", self.access_token))
            .method(method)
            .body(Body::empty())?;

        let response = self.client.request(request).await?;

        if !cb(&response)? {
            return Err(Error::new(anyhow!(
                "onedrive request failed with code: {}",
                response.status()
            )));
        }

        let body = response.into_body();
        let mut de =
            serde_json::Deserializer::from_reader(hyper::body::to_bytes(body).await?.reader());
        let response_body_json = TOut::deserialize(&mut de)?;

        Ok(response_body_json)
    }

    pub async fn request_upload_file<'de>(
        &self,
        upload_session: &UploadSession,
        bytes: &[u8],
        range_in_file: Range<usize>,
        total_file_size: usize,
    ) -> Result<UploadFile> {
        const REQUEST_BASEURL: &'static str = "https://graph.microsoft.com/v1.0/";
        let request = Request::builder()
            .uri(format!("{}", upload_session.upload_url.0))
            .header("Authorization", format!("Bearer {}", self.access_token))
            .header("Content-Length", bytes.len().to_string())
            .header(
                "Content-Range",
                format!(
                    "bytes {}-{}/{}",
                    range_in_file.start, range_in_file.end, total_file_size
                ),
            )
            .method("PUT")
            .body(Body::from(bytes.to_owned()))?;

        let response = self.client.request(request).await?;

        return match response.status() {
            StatusCode::ACCEPTED => {
                let body = response.into_body();
                let mut de = serde_json::Deserializer::from_reader(
                    hyper::body::to_bytes(body).await?.reader(),
                );
                let result = UploadFileChunk::deserialize(&mut de)?;
                Ok(UploadFile::Chunk(result))
            }
            StatusCode::CREATED => {
                let body = response.into_body();
                let mut de = serde_json::Deserializer::from_reader(
                    hyper::body::to_bytes(body).await?.reader(),
                );
                let result = UploadFileFinished::deserialize(&mut de)?;
                Ok(UploadFile::Finished(result))
            }
            _ => Err(Error::new(anyhow!(
                "onedrive request failed with code: {}",
                response.status()
            ))),
        };
    }

    pub async fn get_root_site(&self) -> Result<Site> {
        Ok(self
            .request_empty("GET", "/sites/root", Self::default_cb(&[StatusCode::OK]))
            .await?)
    }

    pub async fn get_drives(&self, site_id: &SiteId) -> Result<ListDrivesRoot> {
        Ok(self
            .request_empty(
                "GET",
                &format!("/sites/{}/drives", site_id.0),
                Self::default_cb(&[StatusCode::OK]),
            )
            .await?)
    }

    pub async fn get_drive(&self, drive_id: &DriveId) -> Result<Drive> {
        Ok(self
            .request_empty(
                "GET",
                &format!("/drives/{}", drive_id.0),
                Self::default_cb(&[StatusCode::OK]),
            )
            .await?)
    }

    // does not do an extra request to get the drive quota
    pub async fn drive_exists_min(&self, site_id: &SiteId) -> Result<ListDrivesValue> {
        let drives = self.get_drives(site_id).await?;
        let drive = drives
            .value
            .into_iter()
            .filter(|drive| {
                if drive.name == Self::DRIVE_NAME_FOR_CONVERSION {
                    true
                } else {
                    false
                }
            })
            .nth(0)
            .ok_or(Error::new(anyhow!("conversion drive not found")))?;

        Ok(drive)
    }

    // does an extra request to get the drive quota
    pub async fn drive_exists(&self, site_id: &SiteId) -> Result<Drive> {
        use futures::future::FutureExt;
        use futures::stream::StreamExt;
        let drive = self
            .get_drives(site_id)
            .then(|x| async move {
                let values = x.map(|x| x.value)?;
                futures::stream::iter(values)
                    .filter_map(|drive| async move {
                        if drive.name == Self::DRIVE_NAME_FOR_CONVERSION {
                            let drive_full = self.get_drive(&drive.id).await.ok()?;
                            Some(drive_full)
                        } else {
                            None
                        }
                    })
                    .take(1)
                    .collect::<Vec<_>>()
                    .await
                    .into_iter()
                    .nth(0)
                    .ok_or(Error::new(anyhow!("conversion drive not found")))
            })
            .await?;
        Ok(drive)
    }

    pub async fn create_upload_file_session(&self, drive_id: &DriveId) -> Result<UploadSession> {
        Ok(self
            .request_empty(
                "POST",
                &format!("/drives/{}/items/root/createUploadSession", drive_id.0),
                Self::default_cb(&[StatusCode::OK]),
            )
            .await?)
    }

    async fn upload_file_lost_chunks(
        &self,
        upload_session: &UploadSession,
        file: &mut tokio::fs::File,
        lost_chunks: Vec<Range<usize>>,
        total_length: usize,
    ) -> Result<UploadFileFinished> {
        debug_assert_ne!(lost_chunks.len(), 0);

        let mut last_lost_chunk_resp = None;

        //TODO: timeout
        loop {
            for lost_chunk in lost_chunks.iter() {
                let size = lost_chunk.end - lost_chunk.start;
                let mut chunk = vec![0u8; size];
                file.read_buf(&mut chunk).await?;

                last_lost_chunk_resp = Some(
                    self.request_upload_file(
                        upload_session,
                        &chunk,
                        lost_chunk.clone(),
                        total_length,
                    )
                    .await?,
                );
            }

            if let Some(ref resp) = last_lost_chunk_resp {
                if let UploadFile::Finished(finished) = resp {
                    return Ok(finished.clone());
                }
            } else {
                std::unreachable!();
            }
        }
    }

    pub async fn upload_file(
        &self,
        upload_session: &UploadSession,
        file_path: &Path,
    ) -> Result<UploadFileFinished> {
        const BUFFER_SIZE: usize = 1024 * 1024;

        let mut file = tokio::fs::File::open(file_path).await?;

        let total_length = file.metadata().await?.len() as usize;
        let mut buffer = vec![0u8; BUFFER_SIZE];
        let mut lost_chunks: Option<Vec<Range<usize>>> = None;

        let mut current_index: usize = 0;
        while let Ok(n) = file.read_buf(&mut buffer).await {
            if n == 0 || current_index >= total_length {
                break;
            }

            let chunk = &buffer[current_index..current_index + n];

            let resp = self
                .request_upload_file(
                    upload_session,
                    chunk,
                    current_index..current_index + n,
                    total_length,
                )
                .await?;

            match resp {
                UploadFile::Chunk(chunk) => {
                    // reached last chunk, and yet we haven't finished. This means some chunks were lost. handle them
                    if current_index + n >= total_length {
                        assert_ne!(chunk.next_expected_ranges.len(), 0);

                        lost_chunks = Some(
                            chunk
                                .next_expected_ranges
                                .into_iter()
                                .map(|x| {
                                    let mut itr = x.split('-');

                                    let a = itr.next().unwrap().parse().unwrap();
                                    let b = if let Some(x) = itr.next() {
                                        x.parse().unwrap()
                                    } else {
                                        total_length
                                    };

                                    a..b
                                })
                                .collect::<Vec<_>>(),
                        );

                        break;
                    } else {
                        current_index += n;
                    }

                    // TODO: logging
                    // println!("uploaded {}%", (current_index as f64 / total_length as f64) * 100.0);
                }
                UploadFile::Finished(finished) => {
                    // TODO: logging
                    // println!("uploaded {}%", (current_index as f64 / total_length as f64) * 100.0);
                    // println!("upload finished");
                    return Ok(finished);
                }
            }
        }

        if let Some(lost_chunks) = lost_chunks {
            self.upload_file_lost_chunks(upload_session, &mut file, lost_chunks, total_length)
                .await
        } else {
            std::unreachable!();
        }
    }

    pub async fn delete_file(&self, drive_id: &DriveId, file: &UploadFileFinished) -> Result<()> {
        Ok(self
            .request_empty(
                "DELETE",
                &format!("/drives/{}/items/{}", drive_id.0, file.id),
                Self::default_cb(&[StatusCode::NO_CONTENT]),
            )
            .await?)
    }

    pub async fn get_url_to_file_as_pdf(&self, file: &UploadFileFinished) -> Result<String> {
        let resp = self
            .request_raw(
                "GET",
                &format!("/drive/items/{}/content?format=pdf", &file.id),
            )
            .await?;

        if resp.status() != StatusCode::FOUND {
            return Err(Error::new(anyhow!("unexpected response")));
        }

        resp.headers()
            .get("Location")
            .map(|x| x.to_str().map(|x| x.to_owned()))
            .ok_or(Error::new(anyhow!("no location header")))?
            .map_err(|e| Error::from(e))
    }
}
