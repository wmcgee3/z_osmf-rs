use zosmf_core::jobs::{Identifier, Status};
use zosmf_macros::Endpoint;

#[derive(Endpoint)]
#[endpoint(method = get, path = "/zosmf/restjobs/jobs/{subsystem}{identifier}")]
pub struct JobStatusBuilder {
    base_url: Box<str>,
    client: reqwest::Client,

    #[endpoint(path)]
    identifier: Identifier,
    #[endpoint(optional, path, setter_fn = "set_subsystem")]
    subsystem: Box<str>,
    #[endpoint(optional, builder_fn = "build_step_data")]
    step_data: Option<bool>,
    #[endpoint(optional, builder_fn = "build_exec_data")]
    exec_data: Option<bool>,
}

impl JobStatusBuilder {
    pub async fn build(self) -> anyhow::Result<Status> {
        let response = self.get_response().await?.error_for_status()?;

        Ok(response.json().await?)
    }
}

fn build_exec_data(
    mut request_builder: reqwest::RequestBuilder,
    builder: &JobStatusBuilder,
) -> reqwest::RequestBuilder {
    if let Some(step_data) = builder.step_data {
        request_builder =
            request_builder.query(&[("exec-data", if step_data { "Y" } else { "N" })]);
    }

    request_builder
}

fn build_step_data(
    mut request_builder: reqwest::RequestBuilder,
    builder: &JobStatusBuilder,
) -> reqwest::RequestBuilder {
    if let Some(step_data) = builder.step_data {
        request_builder =
            request_builder.query(&[("step-data", if step_data { "Y" } else { "N" })]);
    }

    request_builder
}

fn set_subsystem(mut builder: JobStatusBuilder, value: Box<str>) -> JobStatusBuilder {
    builder.subsystem = value;

    builder
}
