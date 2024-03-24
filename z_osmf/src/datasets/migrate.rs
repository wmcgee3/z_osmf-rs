use std::marker::PhantomData;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use z_osmf_macros::{Endpoint, Getters};

use crate::convert::TryFromResponse;
use crate::utils::{get_etag, get_transaction_id};
use crate::ClientCore;

use super::get_member;

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, PartialEq, Serialize)]
pub struct Migrate {
    etag: Option<Box<str>>,
    transaction_id: Box<str>,
}

impl TryFromResponse for Migrate {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, crate::error::Error> {
        let etag = get_etag(&value)?;
        let transaction_id = get_transaction_id(&value)?;

        Ok(Migrate {
            etag,
            transaction_id,
        })
    }
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = put, path = "/zosmf/restfiles/ds/{name}{member}")]
pub struct MigrateBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(path)]
    name: Box<str>,
    #[endpoint(path, builder_fn = build_member)]
    member: Option<Box<str>>,
    #[endpoint(builder_fn = build_body )]
    wait: Option<bool>,

    target_type: PhantomData<T>,
}

#[derive(Serialize)]
struct RequestJson {
    request: &'static str,
    wait: bool,
}

fn build_body<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &MigrateBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    request_builder.json(&RequestJson {
        request: "hmigrate",
        wait: builder.wait == Some(true),
    })
}

fn build_member<T>(builder: &MigrateBuilder<T>) -> String
where
    T: TryFromResponse,
{
    get_member(&builder.member)
}
