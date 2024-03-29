use crate::error::Error;
use crate::utils::get_transaction_id;

#[allow(async_fn_in_trait)]
pub trait TryFromResponse
where
    Self: Sized,
{
    async fn try_from_response(value: reqwest::Response) -> Result<Self, Error>;
}

pub trait TryIntoTarget<T>: Sized {
    async fn try_into_target(self) -> Result<T, Error>;
}

impl<T> TryIntoTarget<T> for reqwest::Response
where
    T: TryFromResponse,
{
    #[inline]
    async fn try_into_target(self) -> Result<T, Error> {
        T::try_from_response(self).await
    }
}

impl TryFromResponse for () {
    async fn try_from_response(_: reqwest::Response) -> Result<Self, Error> {
        Ok(())
    }
}

impl TryFromResponse for String {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, Error> {
        get_transaction_id(&value).map(|v| v.to_string())
    }
}
