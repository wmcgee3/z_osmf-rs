use crate::Result;

#[allow(async_fn_in_trait)]
pub trait TryFromResponse
where
    Self: Sized,
{
    async fn try_from_response(value: reqwest::Response) -> Result<Self>;
}

pub trait TryIntoTarget<T>: Sized {
    async fn try_into_target(self) -> Result<T>;
}

impl<T> TryIntoTarget<T> for reqwest::Response
where
    T: TryFromResponse,
{
    #[inline]
    async fn try_into_target(self) -> Result<T> {
        T::try_from_response(self).await
    }
}

impl TryFromResponse for () {
    async fn try_from_response(_: reqwest::Response) -> Result<Self> {
        Ok(())
    }
}
