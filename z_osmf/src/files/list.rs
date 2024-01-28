use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::runtime::Handle;
use z_osmf_macros::{Endpoint, Getters};

use crate::error::Error;
use crate::utils::get_transaction_id;

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
pub struct FileList {
    items: Box<[FileAttributes]>,
    returned_rows: i32,
    total_rows: i32,
    json_version: i32,
    transaction_id: Box<str>,
}

impl TryFrom<reqwest::Response> for FileList {
    type Error = Error;

    fn try_from(value: reqwest::Response) -> Result<Self, Self::Error> {
        let transaction_id = get_transaction_id(&value)?;

        let json = Handle::current().block_on(value.json())?;

        let ResponseJson {
            items,
            returned_rows,
            total_rows,
            json_version,
        } = json;

        Ok(FileList {
            items,
            returned_rows,
            total_rows,
            json_version,
            transaction_id,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
pub struct FileAttributes {
    name: Box<str>,
    mode: Box<str>,
    size: i32,
    uid: i32,
    #[serde(default)]
    user: Option<Box<str>>,
    gid: i32,
    group: Box<str>,
    mtime: Box<str>,
    #[serde(default)]
    target: Option<Box<str>>,
}

#[derive(Endpoint)]
#[endpoint(method = get, path = "/zosmf/restfiles/fs")]
pub struct FileListBuilder {
    base_url: Arc<str>,
    client: reqwest::Client,

    #[endpoint(query = "path")]
    path: Box<str>,
    #[endpoint(optional, builder_fn = "build_lstat")]
    lstat: bool,
    #[endpoint(optional, query = "group")]
    group: Option<Box<str>>,
    #[endpoint(optional, query = "mtime")]
    mtime: Option<Box<str>>,
    #[endpoint(optional, query = "name")]
    name: Option<Box<str>>,
    #[endpoint(optional, query = "size")]
    size: Option<Box<str>>,
    #[endpoint(optional, query = "perm")]
    perm: Option<Box<str>>,
    #[endpoint(optional, query = "type")]
    file_type: Option<Box<str>>,
    #[endpoint(optional, query = "user")]
    user: Option<Box<str>>,
    #[endpoint(optional, query = "depth")]
    depth: Option<i32>,
    #[endpoint(optional, query = "limit")]
    limit: Option<i32>,
    #[endpoint(optional, query = "filesys")]
    filesys: Option<FileSys>,
    #[endpoint(optional, query = "symlinks")]
    symlinks: Option<SymLinks>,
}

impl FileListBuilder {
    pub async fn build(self) -> Result<FileList, Error> {
        let response = self.get_response().await?;

        response.try_into()
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum FileSys {
    All,
    Same,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SymLinks {
    Follow,
    Report,
}

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
#[serde(rename_all = "camelCase")]
struct ResponseJson {
    items: Box<[FileAttributes]>,
    returned_rows: i32,
    total_rows: i32,
    #[serde(rename = "JSONversion")]
    json_version: i32,
}

fn build_lstat(
    mut request_builder: reqwest::RequestBuilder,
    builder: &FileListBuilder,
) -> reqwest::RequestBuilder {
    if builder.lstat {
        request_builder = request_builder.header("X-IBM-Lstat", "true");
    }

    request_builder
}
