use std::marker::PhantomData;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use z_osmf_macros::{Endpoint, Getters};

use crate::convert::TryFromResponse;
use crate::error::Error;
use crate::utils::get_transaction_id;
use crate::ClientCore;

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
pub struct FileList {
    items: Box<[FileAttributes]>,
    #[getter(copy)]
    returned_rows: i32,
    #[getter(copy)]
    total_rows: i32,
    #[getter(copy)]
    json_version: i32,
    transaction_id: Box<str>,
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

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
pub struct FileAttributes {
    name: Box<str>,
    mode: Box<str>,
    #[getter(copy)]
    size: i32,
    #[getter(copy)]
    uid: i32,
    #[serde(default)]
    user: Option<Box<str>>,
    #[getter(copy)]
    gid: i32,
    group: Box<str>,
    mtime: Box<str>,
    #[serde(default)]
    target: Option<Box<str>>,
}

#[derive(Endpoint)]
#[endpoint(method = get, path = "/zosmf/restfiles/fs")]
pub struct FileListBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

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
    permissions: Option<Box<str>>,
    #[endpoint(optional, query = "type")]
    file_type: Option<ListFileType>,
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
pub enum ListFileType {
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

#[cfg(test)]
mod tests {
    use crate::tests::get_zosmf;

    use super::*;

    #[test]
    fn example_1() {
        let zosmf = get_zosmf();

        let manual_request = zosmf
            .core
            .client
            .get("https://test.com/zosmf/restfiles/fs")
            .query(&[("path", "/usr")])
            .build()
            .unwrap();

        let list_files = zosmf.files().list("/usr").get_request().unwrap();

        assert_eq!(format!("{:?}", manual_request), format!("{:?}", list_files))
    }

    #[test]
    fn example_2() {
        let zosmf = get_zosmf();

        let manual_request = zosmf
            .core
            .client
            .get("https://test.com/zosmf/restfiles/fs")
            .query(&[("path", "/u/ibmuser/myFile.txt")])
            .build()
            .unwrap();

        let list_files = zosmf
            .files()
            .list("/u/ibmuser/myFile.txt")
            .get_request()
            .unwrap();

        assert_eq!(format!("{:?}", manual_request), format!("{:?}", list_files))
    }

    #[test]
    fn example_3() {
        let zosmf = get_zosmf();

        let manual_request = zosmf
            .core
            .client
            .get("https://test.com/zosmf/restfiles/fs")
            .query(&[("path", "/usr/include"), ("name", "f*.h")])
            .build()
            .unwrap();

        let list_files = zosmf
            .files()
            .list("/usr/include")
            .name("f*.h")
            .get_request()
            .unwrap();

        assert_eq!(format!("{:?}", manual_request), format!("{:?}", list_files))
    }

    #[test]
    fn maximal_request() {
        let zosmf = get_zosmf();

        let manual_request = zosmf
            .core
            .client
            .get("https://test.com/zosmf/restfiles/fs")
            .query(&[
                ("path", "/usr/include"),
                ("group", "ibmgrp"),
                ("mtime", "something"),
                ("name", "f*.h"),
                ("size", "10k"),
                ("perm", "755"),
                ("type", "d"),
                ("user", "ibmuser"),
                ("depth", "5"),
                ("limit", "100"),
                ("filesys", "all"),
                ("symlinks", "follow"),
            ])
            .header("X-IBM-Lstat", "true")
            .build()
            .unwrap();

        let request = zosmf
            .files()
            .list("/usr/include")
            .name("f*.h")
            .depth(5)
            .file_system(FileSystem::All)
            .file_type(ListFileType::Directory)
            .group("ibmgrp")
            .limit(100)
            .lstat(true)
            .mtime("something")
            .permissions("755")
            .size("10k")
            .symlinks(SymLinks::Follow)
            .user("ibmuser")
            .get_request()
            .unwrap();

        assert_eq!(format!("{:?}", manual_request), format!("{:?}", request))
    }
}
