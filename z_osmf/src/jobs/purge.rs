use std::marker::PhantomData;
use std::sync::Arc;

use z_osmf_macros::Endpoint;

use crate::convert::TryFromResponse;
use crate::ClientCore;

use super::{AsynchronousResponse, JobIdentifier};

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = delete, path = "/zosmf/restjobs/jobs/{subsystem}{identifier}")]
pub struct PurgeBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

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
            core: self.core,
            subsystem: self.subsystem,
            identifier: self.identifier,
            asynchronous: true,
            target_type: PhantomData,
        }
    }
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

fn set_subsystem<T>(mut builder: PurgeBuilder<T>, value: Box<str>) -> PurgeBuilder<T>
where
    T: TryFromResponse,
{
    builder.subsystem = format!("-{}/", value).into();

    builder
}

#[cfg(test)]
mod tests {
    use crate::tests::*;

    use super::*;

    #[test]
    fn example_1() {
        let zosmf = get_zosmf();

        let manual_request = zosmf
            .core
            .client
            .delete("https://test.com/zosmf/restjobs/jobs/TESTJOBW/JOB00085")
            .header("X-IBM-Job-Modify-Version", "2.0")
            .build()
            .unwrap();

        let identifier = JobIdentifier::NameId("TESTJOBW".into(), "JOB00085".into());
        let job_feedback = zosmf
            .jobs()
            .cancel_and_purge(identifier)
            .get_request()
            .unwrap();

        assert_eq!(
            format!("{:?}", manual_request),
            format!("{:?}", job_feedback)
        )
    }
}
