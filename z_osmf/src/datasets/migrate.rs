use std::marker::PhantomData;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use z_osmf_macros::{Endpoint, Getters};

use crate::convert::TryFromResponse;
use crate::utils::{get_etag, get_transaction_id};
use crate::ClientCore;

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
    #[endpoint(optional, path, setter_fn = set_member)]
    member: Box<str>,
    #[endpoint(optional, builder_fn = build_body )]
    wait: bool,

    #[endpoint(optional, skip_setter, skip_builder)]
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
        wait: builder.wait,
    })
}

fn set_member<T>(mut builder: MigrateBuilder<T>, value: Box<str>) -> MigrateBuilder<T>
where
    T: TryFromResponse,
{
    builder.member = format!("({})", value).into();

    builder
}
