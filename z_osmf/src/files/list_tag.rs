use std::marker::PhantomData;
use std::str::FromStr;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use z_osmf_macros::{Endpoint, Getters};

use crate::convert::TryFromResponse;
use crate::error::Error;
use crate::utils::get_transaction_id;
use crate::ClientCore;

use super::FileTagType;

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
pub struct FileListTag {
    tags: Box<[FileTag]>,
    transaction_id: Box<str>,
}

impl TryFromResponse for FileListTag {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, crate::error::Error> {
        let transaction_id = get_transaction_id(&value)?;

        let ResponseJson { stdout } = value.json().await?;
        let tags = stdout
            .iter()
            .map(|line| FileTag::from_str(line))
            .collect::<Result<Box<[FileTag]>, Error>>()?;

        Ok(FileListTag {
            tags,
            transaction_id,
        })
    }
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = put, path = "/zosmf/restfiles/fs{path}")]
pub struct FileListTagBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(path)]
    path: Box<str>,
    #[endpoint(optional, builder_fn = build_body)]
    recursive: bool,

    #[endpoint(optional, skip_setter, skip_builder)]
    target_type: PhantomData<T>,
}

#[derive(Clone, Debug, Deserialize, Eq, Getters, PartialEq, Serialize)]
pub struct FileTag {
    #[getter(copy)]
    tag_type: Option<FileTagType>,
    code_set: Option<Box<str>>,
    #[getter(copy)]
    text_flag: bool,
    path: Box<str>,
}

impl std::str::FromStr for FileTag {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tag_type = match &s[0..1] {
            "-" => None,
            "b" => Some(FileTagType::Binary),
            "m" => Some(FileTagType::Mixed),
            "t" => Some(FileTagType::Text),
            _ => return Err(Error::Custom("invalid file tag string".into())),
        };
        let code_set = match s[2..14].trim_end() {
            "untagged" => None,
            code_set => Some(code_set.into()),
        };
        let text_flag = s[14..18].trim_end() == "T=on";

        Ok(FileTag {
            tag_type,
            code_set,
            text_flag,
            path: s[20..].into(),
        })
    }
}

#[derive(Serialize)]
struct RequestJson {
    request: &'static str,
    action: &'static str,
    recursive: bool,
}

#[derive(Deserialize)]
struct ResponseJson {
    stdout: Box<[Box<str>]>,
}

fn build_body<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &FileListTagBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    request_builder.json(&RequestJson {
        request: "chtag",
        action: "list",
        recursive: builder.recursive,
    })
}

#[cfg(test)]
mod tests {
    use serde_json::{from_str, Value};

    use crate::tests::{get_zosmf, GetJson};

    use super::*;

    #[test]
    fn file_tag_from_str() {
        assert_eq!(
            FileTag::from_str("b untagged    T=off /tmp/file").unwrap(),
            FileTag {
                tag_type: Some(FileTagType::Binary),
                code_set: None,
                text_flag: false,
                path: "/tmp/file".into(),
            }
        );

        assert_eq!(
            FileTag::from_str("m ISO8859-1   T=off /tmp/file").unwrap(),
            FileTag {
                tag_type: Some(FileTagType::Mixed),
                code_set: Some("ISO8859-1".into()),
                text_flag: false,
                path: "/tmp/file".into(),
            }
        );

        assert_eq!(
            FileTag::from_str("t IBM-1047    T=on  /tmp/file").unwrap(),
            FileTag {
                tag_type: Some(FileTagType::Text),
                code_set: Some("IBM-1047".into()),
                text_flag: true,
                path: "/tmp/file".into(),
            }
        );

        assert_eq!(
            FileTag::from_str("- untagged    T=off /tmp/file").unwrap(),
            FileTag {
                tag_type: None,
                code_set: None,
                text_flag: false,
                path: "/tmp/file".into(),
            }
        );

        assert!(FileTag::from_str("some nonsense").is_err());
    }

    #[test]
    fn maximal_request() {
        let zosmf = get_zosmf();

        let json: Value = from_str(
            r#"
            {
                "request": "chtag",
                "action": "list",
                "recursive": true
            }
        "#,
        )
        .unwrap();
        let manual_request = zosmf
            .core
            .client
            .put("https://test.com/zosmf/restfiles/fs/u/jiahj/testDir")
            .json(&json)
            .build()
            .unwrap();

        let request = zosmf
            .files()
            .list_tag("/u/jiahj/testDir")
            .recursive(true)
            .get_request()
            .unwrap();

        assert_eq!(format!("{:?}", manual_request), format!("{:?}", request));
        assert_eq!(manual_request.json(), request.json());
    }
}
