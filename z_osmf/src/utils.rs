use std::num::NonZeroU32;
use std::str::FromStr;

use reqwest::header::HeaderValue;
use serde::{Deserialize, Serialize};

use crate::error::Error;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum RecordRange {
    StartCount(u32, NonZeroU32),
    StartEnd(Option<u32>, u32),
}

impl From<RecordRange> for HeaderValue {
    fn from(value: RecordRange) -> Self {
        match value {
            RecordRange::StartCount(start, count) => format!("{},{}", start, count),
            RecordRange::StartEnd(Some(start), end) => format!("{}-{}", start, end),
            RecordRange::StartEnd(None, end) => format!("-{}", end),
        }
        .try_into()
        .unwrap()
    }
}

impl FromStr for RecordRange {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(s) = s.strip_prefix('-') {
            return Ok(RecordRange::StartEnd(
                None,
                s.parse()
                    .map_err(|_| Error::Custom("invalid end value".into()))?,
            ));
        }

        if let Some((start, end)) = s.split_once('-') {
            return Ok(RecordRange::StartEnd(
                Some(
                    start
                        .parse()
                        .map_err(|_| Error::Custom("invalid start value".into()))?,
                ),
                end.parse()
                    .map_err(|_| Error::Custom("invalid end value".into()))?,
            ));
        }

        if let Some((start, count)) = s.split_once(',') {
            return Ok(RecordRange::StartCount(
                start
                    .parse()
                    .map_err(|_| Error::Custom("invalid start value".into()))?,
                count
                    .parse()
                    .map_err(|_| Error::Custom("invalid end value".into()))?,
            ));
        }

        Err(Error::Custom("failed to get RecordRange from str".into()))
    }
}

pub(crate) fn get_etag(response: &reqwest::Response) -> Result<Option<Box<str>>, Error> {
    Ok(response
        .headers()
        .get("Etag")
        .map(|v| v.to_str())
        .transpose()?
        .map(|v| v.into()))
}

pub(crate) fn get_transaction_id(response: &reqwest::Response) -> Result<Box<str>, Error> {
    Ok(response
        .headers()
        .get("X-IBM-Txid")
        .ok_or(Error::MissingTransactionId)?
        .to_str()?
        .into())
}

pub(crate) fn de_yes_no<'de, D>(deserializer: D) -> core::result::Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    match s.as_str() {
        "YES" => Ok(true),
        "NO" => Ok(false),
        _ => Err(serde::de::Error::unknown_variant(&s, &["YES", "NO"])),
    }
}

pub(crate) fn ser_yes_no<S>(v: &bool, serializer: S) -> core::result::Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(if *v { "YES" } else { "NO" })
}

pub(crate) fn de_optional_y_n<'de, D>(
    deserializer: D,
) -> core::result::Result<Option<bool>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Ok(Option::<String>::deserialize(deserializer)?
        .map(|s| match s.as_str() {
            "Y" => Ok(true),
            "N" => Ok(false),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Y", "N"])),
        })
        .transpose()?)
}

pub(crate) fn ser_optional_y_n<S>(
    v: &Option<bool>,
    serializer: S,
) -> core::result::Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match v {
        Some(value) => serializer.serialize_str(if *value { "Y" } else { "N" }),
        None => serializer.serialize_none(),
    }
}

pub(crate) fn de_optional_yes_no<'de, D>(
    deserializer: D,
) -> core::result::Result<Option<bool>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Ok(Option::<String>::deserialize(deserializer)?
        .map(|s| match s.as_str() {
            "YES" => Ok(true),
            "NO" => Ok(false),
            _ => Err(serde::de::Error::unknown_variant(&s, &["YES", "NO"])),
        })
        .transpose()?)
}

pub(crate) fn ser_optional_yes_no<S>(
    v: &Option<bool>,
    serializer: S,
) -> core::result::Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match v {
        Some(value) => serializer.serialize_str(if *value { "YES" } else { "NO" }),
        None => serializer.serialize_none(),
    }
}

#[cfg(test)]
mod tests {
    use serde::de::value::StrDeserializer;
    use serde::de::IntoDeserializer;

    use super::*;

    #[test]
    fn test_record_range_into_header_value() {
        let header_value: HeaderValue = RecordRange::StartEnd(Some(0), 249).into();
        assert_eq!(header_value, HeaderValue::from_static("0-249"));

        let header_value: HeaderValue = RecordRange::StartEnd(None, 249).into();
        assert_eq!(header_value, HeaderValue::from_static("-249"));

        let header_value: HeaderValue =
            RecordRange::StartCount(0, NonZeroU32::new(1).unwrap()).into();
        assert_eq!(header_value, HeaderValue::from_static("0,1"));
    }

    #[test]
    fn test_record_range_from_str() {
        let record_range = RecordRange::from_str("0-249").unwrap();
        assert_eq!(record_range, RecordRange::StartEnd(Some(0), 249));

        let record_range = RecordRange::from_str("-249").unwrap();
        assert_eq!(record_range, RecordRange::StartEnd(None, 249));

        let record_range = RecordRange::from_str("0,1").unwrap();
        assert_eq!(
            record_range,
            RecordRange::StartCount(0, NonZeroU32::new(1).unwrap())
        );

        assert!(RecordRange::from_str("-NONSENSE").is_err());

        assert!(RecordRange::from_str("NON-249").is_err());

        assert!(RecordRange::from_str("0-SENSE").is_err());

        assert!(RecordRange::from_str("NON,1").is_err());

        assert!(RecordRange::from_str("0,SENSE").is_err());

        assert!(RecordRange::from_str("NONSENSE").is_err());
    }

    #[test]
    fn test_get_etag() {
        let response = reqwest::Response::from(
            http::Response::builder()
                .header("Etag", "1234")
                .body("")
                .unwrap(),
        );
        assert_eq!(get_etag(&response).unwrap(), Some("1234".into()));

        let response = reqwest::Response::from(http::Response::new(""));
        assert_eq!(get_etag(&response).unwrap(), None);
    }

    #[test]
    fn test_get_transaction_id() {
        let response = reqwest::Response::from(
            http::Response::builder()
                .header("X-IBM-Txid", "1234")
                .body("")
                .unwrap(),
        );
        assert_eq!(get_transaction_id(&response).unwrap(), "1234".into());

        let response = reqwest::Response::from(http::Response::new(""));
        assert!(get_transaction_id(&response).is_err());
    }

    #[test]
    fn test_de_yes_no() {
        let deserializer: StrDeserializer<serde::de::value::Error> = "YES".into_deserializer();
        assert_eq!(de_yes_no(deserializer).unwrap(), true);

        let deserializer: StrDeserializer<serde::de::value::Error> = "NO".into_deserializer();
        assert_eq!(de_yes_no(deserializer).unwrap(), false);

        let deserializer: StrDeserializer<serde::de::value::Error> = "NONSENSE".into_deserializer();
        assert!(de_yes_no(deserializer).is_err());
    }

    #[test]
    fn test_ser_yes_no() {
        let mut serializer = serde_json::Serializer::new(Vec::new());
        ser_yes_no(&true, &mut serializer).unwrap();
        let serialized = String::from_utf8(serializer.into_inner()).unwrap();
        assert_eq!(serialized, r#""YES""#);

        let mut serializer = serde_json::Serializer::new(Vec::new());
        ser_yes_no(&false, &mut serializer).unwrap();
        let serialized = String::from_utf8(serializer.into_inner()).unwrap();
        assert_eq!(serialized, r#""NO""#);
    }

    #[test]
    fn test_de_optional_y_n() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct Test {
            #[serde(default, deserialize_with = "de_optional_y_n")]
            value: Option<bool>,
        }

        assert_eq!(
            serde_json::from_str::<Test>(r#"{"value": "Y"}"#).unwrap(),
            Test { value: Some(true) }
        );

        assert_eq!(
            serde_json::from_str::<Test>(r#"{"value": "N"}"#).unwrap(),
            Test { value: Some(false) }
        );

        assert_eq!(
            serde_json::from_str::<Test>(r#"{"value": null}"#).unwrap(),
            Test { value: None }
        );

        assert_eq!(
            serde_json::from_str::<Test>(r#"{}"#).unwrap(),
            Test { value: None }
        );

        assert!(serde_json::from_str::<Test>(r#"{"value": "NOPE"}"#).is_err());
    }

    #[test]
    fn test_ser_optional_y_n() {
        let mut serializer = serde_json::Serializer::new(Vec::new());
        ser_optional_y_n(&Some(true), &mut serializer).unwrap();
        let serialized = String::from_utf8(serializer.into_inner()).unwrap();
        assert_eq!(serialized, r#""Y""#);

        let mut serializer = serde_json::Serializer::new(Vec::new());
        ser_optional_y_n(&Some(false), &mut serializer).unwrap();
        let serialized = String::from_utf8(serializer.into_inner()).unwrap();
        assert_eq!(serialized, r#""N""#);

        let mut serializer = serde_json::Serializer::new(Vec::new());
        ser_optional_y_n(&None, &mut serializer).unwrap();
        let serialized = String::from_utf8(serializer.into_inner()).unwrap();
        assert_eq!(serialized, r#"null"#);
    }

    #[test]
    fn test_de_optional_yes_no() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct Test {
            #[serde(default, deserialize_with = "de_optional_yes_no")]
            value: Option<bool>,
        }

        assert_eq!(
            serde_json::from_str::<Test>(r#"{"value": "YES"}"#).unwrap(),
            Test { value: Some(true) }
        );

        assert_eq!(
            serde_json::from_str::<Test>(r#"{"value": "NO"}"#).unwrap(),
            Test { value: Some(false) }
        );

        assert_eq!(
            serde_json::from_str::<Test>(r#"{"value": null}"#).unwrap(),
            Test { value: None }
        );

        assert_eq!(
            serde_json::from_str::<Test>(r#"{}"#).unwrap(),
            Test { value: None }
        );

        assert!(serde_json::from_str::<Test>(r#"{"value": "N"}"#).is_err());
    }

    #[test]
    fn test_ser_optional_yes_no() {
        let mut serializer = serde_json::Serializer::new(Vec::new());
        ser_optional_yes_no(&Some(true), &mut serializer).unwrap();
        let serialized = String::from_utf8(serializer.into_inner()).unwrap();
        assert_eq!(serialized, r#""YES""#);

        let mut serializer = serde_json::Serializer::new(Vec::new());
        ser_optional_yes_no(&Some(false), &mut serializer).unwrap();
        let serialized = String::from_utf8(serializer.into_inner()).unwrap();
        assert_eq!(serialized, r#""NO""#);

        let mut serializer = serde_json::Serializer::new(Vec::new());
        ser_optional_yes_no(&None, &mut serializer).unwrap();
        let serialized = String::from_utf8(serializer.into_inner()).unwrap();
        assert_eq!(serialized, r#"null"#);
    }
}
