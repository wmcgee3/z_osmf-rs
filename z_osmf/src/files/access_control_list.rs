use std::marker::PhantomData;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use z_osmf_macros::{Endpoint, Getters};

use crate::convert::TryFromResponse;
use crate::error::Error;
use crate::utils::get_transaction_id;
use crate::ClientCore;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum FileAccessControlType {
    Access,
    #[serde(rename = "dir")]
    Directory,
    File,
}

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
pub struct FileGetAccessControlList {
    name: Box<str>,
    owner: Box<str>,
    group: Box<str>,
    user_access: Option<Box<str>>,
    group_access: Option<Box<str>>,
    other_access: Option<Box<str>>,
    transaction_id: Box<str>,
}

impl TryFromResponse for FileGetAccessControlList {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, crate::error::Error> {
        println!("{:#?}", value.headers());

        let transaction_id = get_transaction_id(&value)?;

        let json: ResponseJson = value.json().await?;

        let mut lines = json.stdout.into_iter();
        let name = lines
            .next()
            .ok_or(Error::Custom(
                "invalid get access control list data: failed to get name".into(),
            ))?
            .trim_start_matches("#file:")
            .trim()
            .into();
        let owner = lines
            .next()
            .ok_or(Error::Custom(
                "invalid get access control list data: failed to get owner".into(),
            ))?
            .trim_start_matches("#owner:")
            .trim()
            .into();
        let group = lines
            .next()
            .ok_or(Error::Custom(
                "invalid get access control list data: failed to get group".into(),
            ))?
            .trim_start_matches("#group:")
            .trim()
            .into();
        let user_access = lines.next().map(|l| l.trim_start_matches("user::").into());
        let group_access = lines.next().map(|l| l.trim_start_matches("group::").into());
        let other_access = lines.next().map(|l| l.trim_start_matches("other::").into());

        Ok(FileGetAccessControlList {
            name,
            owner,
            group,
            user_access,
            group_access,
            other_access,
            transaction_id,
        })
    }
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = put, path = "/zosmf/restfiles/fs{path}")]
pub struct FileGetAccessControlListBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(path)]
    path: Box<str>,
    #[endpoint(optional, builder_fn = build_body)]
    access_control_type: Option<FileAccessControlType>,
    #[endpoint(optional, skip_builder)]
    user: Option<Box<str>>,

    #[endpoint(optional, skip_setter, skip_builder)]
    target_type: PhantomData<T>,
}

#[derive(Serialize)]
struct RequestJson<'a> {
    request: &'static str,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    access_control_type: Option<FileAccessControlType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<&'a str>,
}

#[derive(Deserialize)]
struct ResponseJson {
    stdout: Box<[Box<str>]>,
}

fn build_body<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &FileGetAccessControlListBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    request_builder.json(&RequestJson {
        request: "getfacl",
        access_control_type: builder.access_control_type,
        user: builder.user.as_deref(),
    })
}
