use std::marker::PhantomData;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use z_osmf_macros::Endpoint;

use crate::convert::{TryFromResponse, TryIntoTarget};
use crate::error::Error;
use crate::restfiles::get_transaction_id;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FileList {
    pub items: Box<[FileAttributes]>,
    pub returned_rows: i32,
    pub total_rows: i32,
    pub json_version: i32,
    pub transaction_id: Box<str>,
}

impl TryFromResponse for FileList {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, Error> {
        let transaction_id = get_transaction_id(&value)?;

        let ResponseJson {
            items,
            returned_rows,
            total_rows,
            json_version,
        } = value.json().await?;

        Ok(FileList {
            items,
            returned_rows,
            total_rows,
            json_version,
            transaction_id,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FileAttributes {
    pub name: Box<str>,
    pub mode: Box<str>,
    pub size: i32,
    pub uid: i32,
    #[serde(default)]
    pub user: Option<Box<str>>,
    pub gid: i32,
    pub group: Box<str>,
    pub mtime: Box<str>,
    #[serde(default)]
    pub target: Option<Box<str>>,
}

#[derive(Endpoint)]
#[endpoint(method = get, path = "/zosmf/restfiles/fs")]
pub struct FileListBuilder<T>
where
    T: TryFromResponse,
{
    base_url: Arc<str>,
    client: reqwest::Client,

    #[endpoint(query = "path")]
    path: Box<str>,
    #[endpoint(optional, builder_fn = build_lstat)]
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

    #[endpoint(optional, skip_setter, skip_builder)]
    target_type: PhantomData<T>,
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

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct ResponseJson {
    pub items: Box<[FileAttributes]>,
    pub returned_rows: i32,
    pub total_rows: i32,
    #[serde(rename = "JSONversion")]
    pub json_version: i32,
}

fn build_lstat<T>(
    mut request_builder: reqwest::RequestBuilder,
    builder: &FileListBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    if builder.lstat {
        request_builder = request_builder.header("X-IBM-Lstat", "true");
    }

    request_builder
}
