use std::marker::PhantomData;
use std::sync::Arc;

use serde::Serialize;
use z_osmf_macros::Endpoint;

use crate::convert::TryFromResponse;
use crate::ClientCore;

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = put, path = "/zosmf/restfiles/fs{to_path}")]
pub struct FileRenameBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(builder_fn = build_body)]
    from_path: Box<str>,
    #[endpoint(path)]
    to_path: Box<str>,
    #[endpoint(optional, skip_builder)]
    overwrite: bool,

    #[endpoint(optional, skip_setter, skip_builder)]
    target_type: PhantomData<T>,
}

#[derive(Serialize)]
struct RequestJson<'a> {
    request: &'static str,
    from: &'a str,
    overwrite: bool,
}

fn build_body<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &FileRenameBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    request_builder.json(&RequestJson {
        request: "move",
        from: &builder.from_path,
        overwrite: builder.overwrite,
    })
}

#[cfg(test)]
mod tests {
    use serde_json::{from_str, Value};

    use crate::tests::*;

    use super::*;

    #[test]
    fn overwrite() {
        let zosmf = get_zosmf();

        let json: Value = from_str(
            r#"
            {
                "request": "move",
                "from": "/u/jiahj/sourceFile.txt",
                "overwrite": false
            }
            "#,
        )
        .unwrap();
        let manual_request = zosmf
            .core
            .client
            .put("https://test.com/zosmf/restfiles/fs/u/jiahj/testFile.txt")
            .json(&json)
            .build()
            .unwrap();

        let request = zosmf
            .files()
            .rename("/u/jiahj/sourceFile.txt", "/u/jiahj/testFile.txt")
            .get_request()
            .unwrap();

        assert_eq!(format!("{:?}", manual_request), format!("{:?}", request));

        assert_eq!(manual_request.json(), request.json());
    }
}
