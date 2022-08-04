use std::ops::Range;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;

use anyhow::anyhow;
use dotenv_codegen::dotenv;
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
    client: Arc<Client<HttpConnector>>,
    access_token: String,
    client_id: String,
    client_secret: String,
    refresh_token: String,
    redirect_uri: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshForm<'a> {
    pub client_id: &'a str,
    pub client_secret: &'a str,
    pub refresh_token: &'a str,
    pub redirect_uri: &'a str,
    pub grant_type: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshTokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub scope: String,
    pub token_type: String,
    pub expires_in: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Link {
    UploadUrl(UploadUrl),
    Endpoint(String),
}

impl Link {
    pub fn get(self) -> String {
        const REQUEST_BASEURL: &'static str = "https://graph.microsoft.com/v1.0/";

        match self {
            Link::UploadUrl(upload_url) => upload_url.0,
            Link::Endpoint(endpoint) => format!("{}{}", REQUEST_BASEURL, endpoint),
        }
    }
}

impl Onedrive {
    const _SITE_NAME: &'static str = "organic_notebook";
    const DRIVE_NAME_FOR_CONVERSION: &'static str = "document_conversion";

    pub fn new() -> Self {
        Self {
            client: Arc::new(Client::builder().build_http()),
            access_token: dotenv!("ONEDRIVE_ACCESS_TOKEN").to_owned(),
            client_id: dotenv!("ONEDRIVE_CLIENT_ID").to_owned(),
            client_secret: dotenv!("ONEDRIVE_CLIENT_SECRET").to_owned(),
            refresh_token: dotenv!("ONEDRIVE_REFRESH_TOKEN").to_owned(),
            redirect_uri: dotenv!("ONEDRIVE_REDIRECT_URI").to_owned(),
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

    pub async fn refresh_token(&mut self) -> Result<RefreshTokenResponse> {
        let form = RefreshForm {
            client_id: self.client_id.as_str(),
            client_secret: self.client_secret.as_str(),
            refresh_token: self.refresh_token.as_str(),
            redirect_uri: self.redirect_uri.as_str(),
            grant_type: "refresh_token",
        };

        let req = Request::builder()
            .uri("https://login.microsoftonline.com/common/oauth2/v2.0/token")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(Body::from(serde_urlencoded::to_string(&form).map_err(
                |e| Error::new(anyhow!("Failed to serialize refresh token form: {}", e)),
            )?))?;

        let response = self.client.request(req).await?;
        let mut body_reader = hyper::body::to_bytes(response.into_body()).await?.reader();
        let response: RefreshTokenResponse =
            serde_json::from_reader(&mut body_reader).map_err(|e| {
                Error::new(anyhow!(
                    "Failed to deserialize refresh token response: {}",
                    e
                ))
            })?;

        Ok(response)
    }

    pub async fn request_raw<'de, T: Into<Body> + Clone>(
        &mut self,
        method: &str,
        link: Link,
        is_json: bool,
        body: Option<T>,
        headers: Option<&[(&str, &str)]>,
    ) -> Result<Response<Body>> {
        let mut builder = Request::builder()
            .uri(link.clone().get())
            .header("Authorization", format!("Bearer {}", self.access_token))
            .method(method);

        if is_json {
            builder = builder.header("Content-Type", "application/json");
        }

        if let Some(headers) = headers {
            for header in headers {
                builder = builder.header(header.0.to_owned(), header.1.to_owned());
            }
        }

        let response = self
            .client
            .request(builder.body(if let Some(val) = body.clone() {
                val.into()
            } else {
                Body::empty()
            })?)
            .await?;

        //refresh token
        if response.status() == StatusCode::UNAUTHORIZED {
            let refresh = self.refresh_token().await?;
            self.access_token = refresh.access_token;
            let mut builder = Request::builder()
                .uri(link.get())
                .header("Authorization", format!("Bearer {}", self.access_token))
                .method(method);

            if is_json {
                builder = builder.header("Content-Type", "application/json");
            }

            if let Some(headers) = headers {
                for header in headers {
                    builder = builder.header(header.0.to_owned(), header.1.to_owned());
                }
            }

            let response = self
                .client
                .request(builder.body(if let Some(val) = body {
                    val.into()
                } else {
                    Body::empty()
                })?)
                .await?;

            return Ok(response);
        }

        Ok(response)
    }

    pub async fn request_json_body<'de, TIn, TOut>(
        &mut self,
        method: &str,
        endpoint: &str,
        body: &TIn,
        cb: impl FnOnce(&Response<Body>) -> Result<bool>,
    ) -> Result<TOut>
    where
        TIn: Serialize,
        TOut: Deserialize<'de>,
    {
        let response = self
            .request_raw(
                method,
                Link::Endpoint(endpoint.to_owned()),
                true,
                Some(serde_json::to_string(body)?),
                None,
            )
            .await?;

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
        &mut self,
        method: &str,
        endpoint: &str,
        cb: impl FnOnce(&Response<Body>) -> Result<bool>,
    ) -> Result<TOut>
    where
        TOut: Deserialize<'de>,
    {
        let response = self
            .request_raw(
                method,
                Link::Endpoint(endpoint.to_owned()),
                false,
                Option::<String>::None,
                None,
            )
            .await?;

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
        &mut self,
        upload_session: &UploadSession,
        bytes: &[u8],
        range_in_file: Range<usize>,
        total_file_size: usize,
    ) -> Result<UploadFile> {
        let bytes_len = bytes.len().to_string();
        let content_range = format!(
            "bytes {}-{}/{}",
            range_in_file.start, range_in_file.end, total_file_size
        );
        let headers = [
            ("Content-Length", bytes_len.as_str()),
            ("Content-Range", content_range.as_str()),
        ];

        let response = self
            .request_raw(
                "PUT",
                Link::UploadUrl(upload_session.upload_url.clone()),
                false,
                Option::<String>::None,
                Some(&headers),
            )
            .await?;

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

    pub async fn get_root_site(&mut self) -> Result<Site> {
        Ok(self
            .request_empty("GET", "/sites/root", Self::default_cb(&[StatusCode::OK]))
            .await?)
    }

    pub async fn get_drives(&mut self, site_id: &SiteId) -> Result<ListDrivesRoot> {
        Ok(self
            .request_empty(
                "GET",
                &format!("/sites/{}/drives", site_id.0),
                Self::default_cb(&[StatusCode::OK]),
            )
            .await?)
    }

    pub async fn get_drive(&mut self, drive_id: &DriveId) -> Result<Drive> {
        Ok(self
            .request_empty(
                "GET",
                &format!("/drives/{}", drive_id.0),
                Self::default_cb(&[StatusCode::OK]),
            )
            .await?)
    }

    // does not do an extra request to get the drive quota
    pub async fn drive_exists_min(&mut self, site_id: &SiteId) -> Result<ListDrivesValue> {
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
    pub async fn drive_exists(&mut self, site_id: &SiteId) -> Result<Drive> {
        use futures::stream::StreamExt;
        let values = self.get_drives(site_id).await?.value;
        let cell = Arc::new(Mutex::new(self.clone()));
        let task_cell = cell.clone();
        let drive = futures::stream::iter(values)
            .filter_map(move |drive| {
                let cell = task_cell.clone();
                async move {
                    if drive.name == Self::DRIVE_NAME_FOR_CONVERSION {
                        let drive_full = cell.lock().await.get_drive(&drive.id).await.ok()?;
                        Some(drive_full)
                    } else {
                        None
                    }
                }
            })
            .take(1)
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .nth(0)
            .ok_or(Error::new(anyhow!("conversion drive not found")))?;

        *self = cell.lock().await.clone(); // update self to the latest state in case a refresh happens while mapping drive values. note: probability of that is ~ 1/2^36
        Ok(drive)
    }

    pub async fn create_upload_file_session(
        &mut self,
        drive_id: &DriveId,
    ) -> Result<UploadSession> {
        Ok(self
            .request_empty(
                "POST",
                &format!("/drives/{}/items/root/createUploadSession", drive_id.0),
                Self::default_cb(&[StatusCode::OK]),
            )
            .await?)
    }

    async fn upload_file_lost_chunks(
        &mut self,
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
        &mut self,
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

    pub async fn delete_file(
        &mut self,
        drive_id: &DriveId,
        file: &UploadFileFinished,
    ) -> Result<()> {
        Ok(self
            .request_empty(
                "DELETE",
                &format!("/drives/{}/items/{}", drive_id.0, file.id),
                Self::default_cb(&[StatusCode::NO_CONTENT]),
            )
            .await?)
    }

    pub async fn get_url_to_file_as_pdf(&mut self, file: &UploadFileFinished) -> Result<String> {
        let resp = self
            .request_raw(
                "GET",
                Link::Endpoint(format!("/drive/items/{}/content?format=pdf", &file.id)),
                false,
                Option::<String>::None,
                None,
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
