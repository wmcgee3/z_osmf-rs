use std::marker::PhantomData;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use z_osmf_macros::Endpoint;

use crate::convert::TryFromResponse;
use crate::ClientCore;

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = put, path = "/zosmf/restfiles/fs{to_path}")]
pub struct FileCopyDatasetBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(builder_fn = build_body)]
    from_dataset: Box<str>,
    #[endpoint(path)]
    to_path: Box<str>,
    #[endpoint(optional, skip_builder)]
    from_member: Option<Box<str>>,
    #[endpoint(optional, skip_builder)]
    dataset_type: Option<FileCopyDatasetType>,

    #[endpoint(optional, skip_setter, skip_builder)]
    target_type: PhantomData<T>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum FileCopyDatasetType {
    Binary,
    Executable,
    Text,
}

#[derive(Serialize)]
#[serde(rename_all = "kebab-case")]
struct RequestJson<'a> {
    request: &'static str,
    from_dataset: &'a FromDataset<'a>,
}

#[derive(Serialize)]
struct FromDataset<'a> {
    dsn: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    member: Option<&'a str>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    dataset_type: Option<FileCopyDatasetType>,
}

fn build_body<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &FileCopyDatasetBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    request_builder.json(&RequestJson {
        request: "copy",
        from_dataset: &FromDataset {
            dsn: &builder.from_dataset,
            member: builder.from_member.as_deref(),
            dataset_type: builder.dataset_type,
        },
    })
}

#[cfg(test)]
mod tests {
    use serde_json::{from_str, Value};

    use crate::tests::{get_zosmf, GetJson};

    use super::*;

    #[test]
    fn maximal_request() {
        let zosmf = get_zosmf();

        let json: Value = from_str(
            r#"
            {
                "request": "copy",
                "from-dataset": {
                    "dsn": "MY.TEST.PDS",
                    "member": "TEST",
                    "type": "text"
                }
            }
            "#,
        )
        .unwrap();
        let manual_request = zosmf
            .core
            .client
            .put("https://test.com/zosmf/restfiles/fs/u/jiahj/copyFile.txt")
            .json(&json)
            .build()
            .unwrap();

        let request = zosmf
            .files()
            .copy_dataset("MY.TEST.PDS", "/u/jiahj/copyFile.txt")
            .from_member("TEST")
            .dataset_type(FileCopyDatasetType::Text)
            .get_request()
            .unwrap();

        assert_eq!(format!("{:?}", manual_request), format!("{:?}", request))
    }

    #[test]
    fn minimal_request() {
        let zosmf = get_zosmf();

        let json: Value = from_str(
            r#"
            {
                "request": "copy",
                "from-dataset": {
                    "dsn": "MY.TEST.DS"
                }
            }
            "#,
        )
        .unwrap();
        let manual_request = zosmf
            .core
            .client
            .put("https://test.com/zosmf/restfiles/fs/u/jiahj/copyFile.txt")
            .json(&json)
            .build()
            .unwrap();

        let request = zosmf
            .files()
            .copy_dataset("MY.TEST.DS", "/u/jiahj/copyFile.txt")
            .get_request()
            .unwrap();

        assert_eq!(format!("{:?}", manual_request), format!("{:?}", request));
        assert_eq!(manual_request.json(), request.json())
    }
}
