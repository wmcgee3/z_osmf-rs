use anyhow::Context;
use reqwest::Response;
use serde::Deserialize;

pub(crate) fn get_etag(response: &Response) -> anyhow::Result<Option<String>> {
    Ok(response
        .headers()
        .get("Etag")
        .map(|v| v.to_str())
        .transpose()?
        .map(|v| v.to_string()))
}

pub(crate) fn get_session_ref(response: &Response) -> anyhow::Result<Option<String>> {
    Ok(response
        .headers()
        .get("X-IBM-Session-Ref")
        .map(|v| v.to_str())
        .transpose()?
        .map(|v| v.to_string()))
}

pub(crate) fn get_transaction_id(response: &Response) -> anyhow::Result<String> {
    Ok(response
        .headers()
        .get("X-IBM-Txid")
        .context("missing transaction id")?
        .to_str()?
        .to_string())
}

pub(crate) fn de_yes_no<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    Ok(s == "YES")
}

pub(crate) fn ser_yes_no<S>(v: &bool, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(if *v { "YES" } else { "NO" })
}

pub(crate) fn de_optional_y_n<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Ok(Option::<String>::deserialize(deserializer)?.map(|s| s == "Y"))
}

pub(crate) fn ser_optional_y_n<S>(v: &Option<bool>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match v {
        Some(value) => serializer.serialize_str(if *value { "YES" } else { "NO" }),
        None => serializer.serialize_none(),
    }
}

pub(crate) fn de_optional_yes_no<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Ok(Option::<String>::deserialize(deserializer)?.map(|s| s == "YES"))
}

pub(crate) fn ser_optional_yes_no<S>(v: &Option<bool>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match v {
        Some(value) => serializer.serialize_str(if *value { "YES" } else { "NO" }),
        None => serializer.serialize_none(),
    }
}
