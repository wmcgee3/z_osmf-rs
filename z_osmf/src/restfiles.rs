use crate::error::Error;

pub struct Binary;
pub struct Record;
pub struct Text;

pub struct Etag;
pub struct NoEtag;

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
