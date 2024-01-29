use std::num::NonZeroU32;
use std::str::FromStr;

use reqwest::header::HeaderValue;
use serde::Deserialize;

use crate::error::Error;

#[derive(Clone, Copy, Debug)]
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

    Ok(s == "YES")
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
    Ok(Option::<String>::deserialize(deserializer)?.map(|s| s == "Y"))
}

pub(crate) fn ser_optional_y_n<S>(
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

pub(crate) fn de_optional_yes_no<'de, D>(
    deserializer: D,
) -> core::result::Result<Option<bool>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Ok(Option::<String>::deserialize(deserializer)?.map(|s| s == "YES"))
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
