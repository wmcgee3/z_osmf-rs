use reqwest::Response;
use serde::Deserialize;
use z_osmf_core::error::Error;

pub(crate) fn get_etag(response: &Response) -> Result<Option<Box<str>>, Error> {
    Ok(response
        .headers()
        .get("Etag")
        .map(|v| v.to_str())
        .transpose()?
        .map(|v| v.into()))
}

pub(crate) fn get_session_ref(response: &Response) -> Result<Option<Box<str>>, Error> {
    Ok(response
        .headers()
        .get("X-IBM-Session-Ref")
        .map(|v| v.to_str())
        .transpose()?
        .map(|v| v.into()))
}

pub(crate) fn get_transaction_id(response: &Response) -> Result<Box<str>, Error> {
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
