use std::marker::PhantomData;
use std::str::FromStr;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use z_osmf_macros::{Endpoint, Getters};

use crate::convert::TryFromResponse;
use crate::error::Error;
use crate::utils::get_transaction_id;
use crate::ClientCore;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Links {
    Change,
    Suppress,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TagType {
    Binary,
    Mixed,
    Text,
}

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, PartialEq, Serialize)]
pub struct Tags {
    tags: Box<[Tag]>,
    transaction_id: Box<str>,
}

impl TryFromResponse for Tags {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, crate::error::Error> {
        let transaction_id = get_transaction_id(&value)?;

        let TagsResponseJson { stdout } = value.json().await?;
        let tags = stdout
            .iter()
            .map(|line| Tag::from_str(line))
            .collect::<Result<Box<[Tag]>, Error>>()?;

        Ok(Tags {
            tags,
            transaction_id,
        })
    }
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = put, path = "/zosmf/restfiles/fs{path}")]
pub struct TagsBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(path)]
    path: Box<str>,
    #[endpoint(optional, builder_fn = build_tags_body)]
    recursive: bool,

    #[endpoint(optional, skip_setter, skip_builder)]
    target_type: PhantomData<T>,
}

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, PartialEq, Serialize)]
pub struct Tag {
    #[getter(copy)]
    tag_type: Option<TagType>,
    code_set: Option<Box<str>>,
    #[getter(copy)]
    text_flag: bool,
    path: Box<str>,
}

impl std::str::FromStr for Tag {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tag_type = match &s[0..1] {
            "-" => None,
            "b" => Some(TagType::Binary),
            "m" => Some(TagType::Mixed),
            "t" => Some(TagType::Text),
            _ => return Err(Error::Custom("invalid file tag string".into())),
        };
        let code_set = match s[2..14].trim_end() {
            "untagged" => None,
            code_set => Some(code_set.into()),
        };
        let text_flag = s[14..18].trim_end() == "T=on";

        Ok(Tag {
            tag_type,
            code_set,
            text_flag,
            path: s[20..].into(),
        })
    }
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = put, path = "/zosmf/restfiles/fs{path}")]
pub struct RemoveBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(path)]
    path: Box<str>,
    #[endpoint(optional, builder_fn = build_remove_body)]
    links: Option<Links>,
    #[endpoint(optional, skip_builder)]
    recursive: bool,

    #[endpoint(optional, skip_setter, skip_builder)]
    target_type: PhantomData<T>,
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = put, path = "/zosmf/restfiles/fs{path}")]
pub struct SetBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(path)]
    path: Box<str>,
    #[endpoint(optional, builder_fn = build_set_body)]
    tag_type: Option<TagType>,
    #[endpoint(optional, skip_builder)]
    code_set: Option<Box<str>>,
    #[endpoint(optional, skip_builder)]
    links: Option<Links>,
    #[endpoint(optional, skip_builder)]
    recursive: bool,

    #[endpoint(optional, skip_setter, skip_builder)]
    target_type: PhantomData<T>,
}

#[derive(Serialize)]
struct TagsRequestJson {
    request: &'static str,
    action: &'static str,
    recursive: bool,
}

#[derive(Deserialize)]
struct TagsResponseJson {
    stdout: Box<[Box<str>]>,
}

fn build_tags_body<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &TagsBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    request_builder.json(&TagsRequestJson {
        request: "chtag",
        action: "list",
        recursive: builder.recursive,
    })
}

#[derive(Serialize)]
struct RemoveRequestJson {
    request: &'static str,
    action: &'static str,
    links: Option<Links>,
    recursive: bool,
}

fn build_remove_body<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &RemoveBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    request_builder.json(&RemoveRequestJson {
        request: "chtag",
        action: "remove",
        links: builder.links,
        recursive: builder.recursive,
    })
}

#[derive(Debug, Serialize)]
struct SetRequestJson<'a> {
    request: &'static str,
    action: &'static str,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    tag_type: Option<TagType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    codeset: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    links: Option<Links>,
    recursive: bool,
}

fn build_set_body<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &SetBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    request_builder.json(&SetRequestJson {
        request: "chtag",
        action: "set",
        tag_type: builder.tag_type,
        codeset: builder.code_set.as_deref(),
        links: builder.links,
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
            Tag::from_str("b untagged    T=off /tmp/file").unwrap(),
            Tag {
                tag_type: Some(TagType::Binary),
                code_set: None,
                text_flag: false,
                path: "/tmp/file".into(),
            }
        );

        assert_eq!(
            Tag::from_str("m ISO8859-1   T=off /tmp/file").unwrap(),
            Tag {
                tag_type: Some(TagType::Mixed),
                code_set: Some("ISO8859-1".into()),
                text_flag: false,
                path: "/tmp/file".into(),
            }
        );

        assert_eq!(
            Tag::from_str("t IBM-1047    T=on  /tmp/file").unwrap(),
            Tag {
                tag_type: Some(TagType::Text),
                code_set: Some("IBM-1047".into()),
                text_flag: true,
                path: "/tmp/file".into(),
            }
        );

        assert_eq!(
            Tag::from_str("- untagged    T=off /tmp/file").unwrap(),
            Tag {
                tag_type: None,
                code_set: None,
                text_flag: false,
                path: "/tmp/file".into(),
            }
        );

        assert!(Tag::from_str("some nonsense").is_err());
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

    #[test]
    fn maximal_remove() {
        let zosmf = get_zosmf();

        let json: Value = from_str(
            r#"
            {
                "request": "chtag",
                "action": "remove",
                "links": "suppress",
                "recursive": true
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
            .remove_tag("/u/jiahj/testFile.txt")
            .links(Links::Suppress)
            .recursive(true)
            .get_request()
            .unwrap();

        assert_eq!(format!("{:?}", manual_request), format!("{:?}", request));
        assert_eq!(manual_request.json(), request.json());
    }

    #[test]
    fn maximal_set() {
        let zosmf = get_zosmf();

        let json: Value = from_str(
            r#"
            {
                "request": "chtag",
                "action": "set",
                "recursive": false,
                "type": "mixed",
                "codeset": "IBM-1047",
                "links": "suppress"
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
            .set_tag("/u/jiahj/testFile.txt")
            .tag_type(TagType::Mixed)
            .code_set("IBM-1047")
            .links(Links::Suppress)
            .get_request()
            .unwrap();

        assert_eq!(format!("{:?}", manual_request), format!("{:?}", request));
        assert_eq!(manual_request.json(), request.json());
    }
}
