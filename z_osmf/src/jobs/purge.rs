use std::marker::PhantomData;
use std::sync::Arc;

use z_osmf_macros::Endpoint;

use crate::convert::TryFromResponse;
use crate::ClientCore;

use super::{get_subsystem, Identifier};

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = delete, path = "/zosmf/restjobs/jobs{subsystem}/{identifier}")]
pub struct PurgeBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(path, builder_fn = build_subsystem)]
    subsystem: Option<Box<str>>,
    #[endpoint(path)]
    identifier: Identifier,
    #[endpoint(skip_setter, builder_fn = build_asynchronous)]
    asynchronous: Option<bool>,

    target_type: PhantomData<T>,
}

impl<T> PurgeBuilder<T>
where
    T: TryFromResponse,
{
    pub fn asynchronous(self) -> PurgeBuilder<()> {
        PurgeBuilder {
            core: self.core,
            subsystem: self.subsystem,
            identifier: self.identifier,
            asynchronous: Some(true),
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
        if builder.asynchronous == Some(true) {
            "1.0"
        } else {
            "2.0"
        },
    )
}

fn build_subsystem<T>(builder: &PurgeBuilder<T>) -> String
where
    T: TryFromResponse,
{
    get_subsystem(&builder.subsystem)
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

        let identifier = Identifier::NameId("TESTJOBW".into(), "JOB00085".into());
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
