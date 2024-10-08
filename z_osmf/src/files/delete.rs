use std::marker::PhantomData;
use std::sync::Arc;

use z_osmf_macros::Endpoint;

use crate::convert::TryFromResponse;
use crate::ClientCore;

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = delete, path = "/zosmf/restfiles/fs{path}")]
pub struct FileDeleteBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(path)]
    path: Arc<str>,
    #[endpoint(builder_fn = build_recursive)]
    recursive: Option<bool>,

    target_type: PhantomData<T>,
}

fn build_recursive<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &FileDeleteBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    match builder.recursive {
        Some(true) => request_builder.header("X-IBM-Option", "recursive"),
        _ => request_builder,
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::*;

    #[test]
    fn example_1() {
        let zosmf = get_zosmf();

        let manual_request = zosmf
            .core
            .client
            .delete("https://test.com/zosmf/restfiles/fs/u/jiahj/text.txt")
            .build()
            .unwrap();

        let delete_file = zosmf
            .files()
            .delete("/u/jiahj/text.txt")
            .get_request()
            .unwrap();

        assert_eq!(
            format!("{:?}", manual_request),
            format!("{:?}", delete_file)
        )
    }

    #[test]
    fn example_2() {
        let zosmf = get_zosmf();

        let manual_request = zosmf
            .core
            .client
            .delete("https://test.com/zosmf/restfiles/fs/u/jiahj/testDir")
            .header("X-IBM-Option", "recursive")
            .build()
            .unwrap();

        let delete_file = zosmf
            .files()
            .delete("/u/jiahj/testDir")
            .recursive(true)
            .get_request()
            .unwrap();

        assert_eq!(
            format!("{:?}", manual_request),
            format!("{:?}", delete_file)
        )
    }
}
