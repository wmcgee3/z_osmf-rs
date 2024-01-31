use std::marker::PhantomData;
use std::sync::Arc;

use z_osmf_macros::Endpoint;

use crate::convert::{TryFromResponse, TryIntoTarget};

use super::{AsynchronousResponse, JobIdentifier};

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = delete, path = "/zosmf/restjobs/jobs/{subsystem}{identifier}")]
pub struct PurgeBuilder<T>
where
    T: TryFromResponse,
{
    base_url: Arc<str>,
    client: reqwest::Client,

    #[endpoint(optional, path, setter_fn = set_subsystem)]
    subsystem: Box<str>,
    #[endpoint(path)]
    identifier: JobIdentifier,
    #[endpoint(optional, skip_setter, builder_fn = build_asynchronous)]
    asynchronous: bool,

    #[endpoint(optional, skip_setter, skip_builder)]
    target_type: PhantomData<T>,
}

impl<T> PurgeBuilder<T>
where
    T: TryFromResponse,
{
    pub fn asynchronous(self) -> PurgeBuilder<AsynchronousResponse> {
        PurgeBuilder {
            base_url: self.base_url,
            client: self.client,
            subsystem: self.subsystem,
            identifier: self.identifier,
            asynchronous: true,
            target_type: PhantomData,
        }
    }
}

fn set_subsystem<T>(mut builder: PurgeBuilder<T>, value: Box<str>) -> PurgeBuilder<T>
where
    T: TryFromResponse,
{
    builder.subsystem = format!("-{}/", value).into();

    builder
}

fn build_asynchronous<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &PurgeBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    request_builder.header(
        "X-IBM-Job-Modify-Version",
        if builder.asynchronous { "1.0" } else { "2.0" },
    )
}
