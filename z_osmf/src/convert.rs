use crate::error::Error;

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
