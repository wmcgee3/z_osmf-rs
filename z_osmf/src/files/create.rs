use std::marker::PhantomData;
use std::sync::Arc;

use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};
use z_osmf_macros::Endpoint;

use crate::convert::TryFromResponse;
use crate::ClientCore;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CreateFileType {
    Directory,
    File,
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = post, path = "/zosmf/restfiles/fs{path}")]
pub struct FileCreateBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(path)]
    path: Box<str>,

    #[endpoint(optional, builder_fn = build_body)]
    file_type: Option<CreateFileType>,
    #[endpoint(optional, skip_builder)]
    mode: Option<Box<str>>,

    #[endpoint(optional, skip_setter, skip_builder)]
    target_type: PhantomData<T>,
}

#[derive(Serialize)]
struct RequestJson<'a> {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    file_type: Option<&'a CreateFileType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<&'a str>,
}

fn build_body<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &FileCreateBuilder<T>,
) -> RequestBuilder
where
    T: TryFromResponse,
{
    request_builder.json(&RequestJson {
        file_type: builder.file_type.as_ref(),
        mode: builder.mode.as_deref(),
    })
}

#[cfg(test)]
mod tests {
    use crate::tests::*;

    use super::*;

    #[test]
    fn example_1() {
        let zosmf = get_zosmf();

        let raw_json = r#"
        {
            "type": "file",
            "mode": "RWXRW-RW-"
        }
        "#;
        let json: serde_json::Value = serde_json::from_str(raw_json).unwrap();

        let manual_request = zosmf
            .core
            .client
            .post("https://test.com/zosmf/restfiles/fs/u/jiahj/text.txt")
            .json(&json)
            .build()
            .unwrap();

        let create_file = zosmf
            .files()
            .create("/u/jiahj/text.txt")
            .file_type(CreateFileType::File)
            .mode("RWXRW-RW-")
            .get_request()
            .unwrap();

        assert_eq!(
            format!("{:?}", manual_request),
            format!("{:?}", create_file)
        );

        assert_eq!(manual_request.json(), create_file.json())
    }

    #[test]
    fn example_2() {
        let zosmf = get_zosmf();

        let raw_json = r#"
        {
            "type": "directory",
            "mode": "rwxr-xrwx"
        }
        "#;
        let json: serde_json::Value = serde_json::from_str(raw_json).unwrap();

        let manual_request = zosmf
            .core
            .client
            .post("https://test.com/zosmf/restfiles/fs/u/jiahj/testDir")
            .json(&json)
            .build()
            .unwrap();

        let create_file = zosmf
            .files()
            .create("/u/jiahj/testDir")
            .file_type(CreateFileType::Directory)
            .mode("rwxr-xrwx")
            .get_request()
            .unwrap();

        assert_eq!(
            format!("{:?}", manual_request),
            format!("{:?}", create_file)
        );

        assert_eq!(manual_request.json(), create_file.json())
    }
}
