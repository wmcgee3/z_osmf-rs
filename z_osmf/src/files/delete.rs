use std::marker::PhantomData;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use z_osmf_macros::{Endpoint, Getters};

use crate::convert::TryFromResponse;
use crate::error::Error;
use crate::utils::get_transaction_id;

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
pub struct FileDelete {
    transaction_id: Box<str>,
}

impl TryFromResponse for FileDelete {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, Error> {
        let transaction_id = get_transaction_id(&value)?;

        Ok(FileDelete { transaction_id })
    }
}

#[derive(Endpoint)]
#[endpoint(method = delete, path = "/zosmf/restfiles/fs{path}")]
pub struct FileDeleteBuilder<T>
where
    T: TryFromResponse,
{
    base_url: Arc<str>,
    client: reqwest::Client,

    #[endpoint(path)]
    path: Box<str>,
    #[endpoint(optional, builder_fn = build_recursive)]
    recursive: bool,

    #[endpoint(optional, skip_setter, skip_builder)]
    target_type: PhantomData<T>,
}

fn build_recursive<T>(
    mut request_builder: reqwest::RequestBuilder,
    builder: &FileDeleteBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    if builder.recursive {
        request_builder = request_builder.header("X-IBM-Option", "recursive");
    }

    request_builder
}

#[cfg(test)]
mod tests {
    use crate::tests::*;

    #[test]
    fn example_1() {
        let zosmf = get_zosmf();

        let manual_request = zosmf
            .client
            .delete("https://test.com/zosmf/restfiles/fs/u/jiahj/text.txt")
            .build()
            .unwrap();

        let delete_file = zosmf
            .delete_file("/u/jiahj/text.txt")
            .get_request()
            .unwrap();

        assert_eq!(
            format!("{:?}", manual_request),
            format!("{:?}", delete_file)
        )
    }

    #[test]
    fn example_2() {
        let zosmf = get_zosmf();

        let manual_request = zosmf
            .client
            .delete("https://test.com/zosmf/restfiles/fs/u/jiahj/testDir")
            .build()
            .unwrap();

        let delete_file = zosmf.delete_file("/u/jiahj/testDir").get_request().unwrap();

        assert_eq!(
            format!("{:?}", manual_request),
            format!("{:?}", delete_file)
        )
    }
}
