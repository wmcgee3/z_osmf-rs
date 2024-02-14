use std::marker::PhantomData;
use std::sync::Arc;

use serde::Serialize;
use z_osmf_macros::Endpoint;

use crate::convert::TryFromResponse;
use crate::ClientCore;

pub struct FileUnlink {}

impl TryFromResponse for FileUnlink {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, crate::error::Error> {
        println!("{:#?}", value.headers());
        println!("{}", value.text().await?);

        Ok(FileUnlink {})
    }
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = put, path = "/zosmf/restfiles/fs{path}")]
pub struct FileUnlinkBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(path)]
    path: Box<str>,

    #[endpoint(optional, skip_setter, builder_fn = build_body)]
    target_type: PhantomData<T>,
}

#[derive(Serialize)]
struct RequestJson {
    request: &'static str,
}

fn build_body<T>(
    request_builder: reqwest::RequestBuilder,
    _builder: &FileUnlinkBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    request_builder.json(&RequestJson { request: "unlink" })
}

#[cfg(test)]
mod tests {
    use serde_json::{from_str, Value};

    use crate::tests::*;

    #[test]
    fn requests() {
        let zosmf = get_zosmf();

        let json: Value = from_str(
            r#"
            {
                "request": "unlink"
            }
            "#,
        )
        .unwrap();
        let manual_request = zosmf
            .core
            .client
            .put("https://test.com/zosmf/restfiles/fs/u/jiahj/targetFile.txt")
            .json(&json)
            .build()
            .unwrap();

        let request = zosmf
            .files()
            .unlink("/u/jiahj/targetFile.txt")
            .get_request()
            .unwrap();

        assert_eq!(format!("{:?}", manual_request), format!("{:?}", request));
        assert_eq!(manual_request.json(), request.json());
    }
}
