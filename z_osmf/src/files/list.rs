use std::marker::PhantomData;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use z_osmf_macros::{Endpoint, Getters};

use crate::convert::{TryFromResponse, TryIntoTarget};
use crate::error::Error;
use crate::utils::get_transaction_id;

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
pub struct ListFiles {
    items: Box<[FileAttributes]>,
    returned_rows: i32,
    total_rows: i32,
    json_version: i32,
    transaction_id: Box<str>,
}

impl TryFromResponse for ListFiles {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, Error> {
        let transaction_id = get_transaction_id(&value)?;

        let ResponseJson {
            items,
            returned_rows,
            total_rows,
            json_version,
        } = value.json().await?;

        Ok(ListFiles {
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

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum FileType {
    #[serde(rename = "c")]
    CharacterSpecialFile,
    #[serde(rename = "d")]
    Directory,
    #[serde(rename = "p")]
    FIFO,
    #[serde(rename = "f")]
    File,
    #[serde(rename = "s")]
    Socket,
    #[serde(rename = "l")]
    SymbolicLink,
}

#[derive(Endpoint)]
#[endpoint(method = get, path = "/zosmf/restfiles/fs")]
pub struct ListFilesBuilder<T>
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
    file_type: Option<FileType>,
    #[endpoint(optional, query = "user")]
    user: Option<Box<str>>,
    #[endpoint(optional, query = "depth")]
    depth: Option<i32>,
    #[endpoint(optional, query = "limit")]
    limit: Option<i32>,
    #[endpoint(optional, query = "filesys")]
    file_system: Option<FileSystem>,
    #[endpoint(optional, query = "symlinks")]
    symlinks: Option<SymLinks>,

    #[endpoint(optional, skip_setter, skip_builder)]
    target_type: PhantomData<T>,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum FileSystem {
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
    items: Box<[FileAttributes]>,
    returned_rows: i32,
    total_rows: i32,
    #[serde(rename = "JSONversion")]
    json_version: i32,
}

fn build_lstat<T>(
    mut request_builder: reqwest::RequestBuilder,
    builder: &ListFilesBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    if builder.lstat {
        request_builder = request_builder.header("X-IBM-Lstat", "true");
    }

    request_builder
}
