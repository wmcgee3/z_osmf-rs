use std::marker::PhantomData;
use std::sync::Arc;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use z_osmf_macros::{Endpoint, Getters};

use crate::convert::TryFromResponse;
use crate::error::Error;
use crate::restfiles::get_transaction_id;
use crate::ClientCore;

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
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

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
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
    #[getter(copy)]
    mtime: NaiveDateTime,
    #[serde(default)]
    target: Option<Box<str>>,
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = get, path = "/zosmf/restfiles/fs")]
pub struct FilesBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(query = "path")]
    path: Box<str>,
    #[endpoint(builder_fn = build_lstat)]
    lstat: Option<bool>,
    #[endpoint(query = "group")]
    group: Option<Box<str>>,
    #[endpoint(query = "mtime")]
    modified_days: Option<Filter<u32>>,
    #[endpoint(query = "name")]
    name: Option<Box<str>>,
    #[endpoint(query = "size")]
    size: Option<Filter<FileSize>>,
    #[endpoint(query = "perm")]
    permissions: Option<Box<str>>,
    #[endpoint(query = "type")]
    file_type: Option<FileType>,
    #[endpoint(query = "user")]
    user: Option<Box<str>>,
    #[endpoint(query = "depth")]
    depth: Option<i32>,
    #[endpoint(query = "limit")]
    limit: Option<i32>,
    #[endpoint(query = "filesys")]
    file_system: Option<FileSystem>,
    #[endpoint(query = "symlinks")]
    symlinks: Option<SymLinks>,

    target_type: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Filter<T>
where
    T: std::fmt::Display + std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    LessThan(T),
    EqualTo(T),
    GreaterThan(T),
}

impl<'de, T> Deserialize<'de> for Filter<T>
where
    T: std::fmt::Display + std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        let v = match s {
            s if s.starts_with('+') => Filter::GreaterThan(
                T::from_str(s.trim_start_matches('+')).map_err(serde::de::Error::custom)?,
            ),
            s if s.starts_with('-') => Filter::LessThan(
                T::from_str(s.trim_start_matches('-')).map_err(serde::de::Error::custom)?,
            ),
            s => Filter::EqualTo(T::from_str(&s).map_err(serde::de::Error::custom)?),
        };

        Ok(v)
    }
}

impl<T> Serialize for Filter<T>
where
    T: std::fmt::Display + std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = match self {
            Filter::EqualTo(f) => format!("{}", f),
            Filter::GreaterThan(f) => format!("+{}", f),
            Filter::LessThan(f) => format!("-{}", f),
        };

        serializer.serialize_str(&s)
    }
}

// TODO: impl serde?
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FileSize {
    Bytes(u32),
    Kilobytes(u32),
    Megabytes(u32),
    Gigabytes(u32),
}

impl std::fmt::Display for FileSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileSize::Bytes(s) => write!(f, "{}", s),
            FileSize::Kilobytes(s) => write!(f, "{}K", s),
            FileSize::Megabytes(s) => write!(f, "{}M", s),
            FileSize::Gigabytes(s) => write!(f, "{}G", s),
        }
    }
}

impl std::str::FromStr for FileSize {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = match s {
            s if s.ends_with('K') => FileSize::Kilobytes(u32::from_str(s.trim_end_matches('K'))?),
            s if s.ends_with('M') => FileSize::Megabytes(u32::from_str(s.trim_end_matches('M'))?),
            s if s.ends_with('G') => FileSize::Gigabytes(u32::from_str(s.trim_end_matches('G'))?),
            s => FileSize::Bytes(u32::from_str(s)?),
        };

        Ok(v)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum FileSystem {
    All,
    Same,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
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

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
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
    request_builder: reqwest::RequestBuilder,
    builder: &FilesBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    match builder.lstat {
        Some(true) => request_builder.header("X-IBM-Lstat", "true"),
        _ => request_builder,
    }
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
                ("mtime", "1"),
                ("name", "f*.h"),
                ("size", "10K"),
                ("perm", "755"),
                ("type", "f"),
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
            .file_type(FileType::File)
            .group("ibmgrp")
            .limit(100)
            .lstat(true)
            .modified_days(Filter::EqualTo(1))
            .permissions("755")
            .size(Filter::EqualTo(FileSize::Kilobytes(10)))
            .symlinks(SymLinks::Follow)
            .user("ibmuser")
            .get_request()
            .unwrap();

        assert_eq!(format!("{:?}", manual_request), format!("{:?}", request))
    }
}
