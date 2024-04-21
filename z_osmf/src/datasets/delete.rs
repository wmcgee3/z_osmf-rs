use std::marker::PhantomData;
use std::sync::Arc;

use z_osmf_macros::Endpoint;

use crate::convert::TryFromResponse;
use crate::ClientCore;

use super::{get_member, get_volume};

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = delete, path = "/zosmf/restfiles/ds{volume}/{dataset}{member}")]
pub struct DatasetDeleteBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(path)]
    dataset: Box<str>,
    #[endpoint(path, builder_fn = build_volume)]
    volume: Option<Box<str>>,
    #[endpoint(path, builder_fn = build_member)]
    member: Option<Box<str>>,
    #[endpoint(header = "X-IBM-Dsname-Encoding")]
    dsname_encoding: Option<Box<str>>,

    target_type: PhantomData<T>,
}

fn build_member<T>(builder: &DatasetDeleteBuilder<T>) -> String
where
    T: TryFromResponse,
{
    get_member(&builder.member)
}

fn build_volume<T>(builder: &DatasetDeleteBuilder<T>) -> String
where
    T: TryFromResponse,
{
    get_volume(&builder.volume)
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
            .delete("https://test.com/zosmf/restfiles/ds/JIAHJ.REST.TEST.DATASET")
            .build()
            .unwrap();

        let delete_dataset = zosmf
            .datasets()
            .delete("JIAHJ.REST.TEST.DATASET")
            .get_request()
            .unwrap();

        assert_eq!(
            format!("{:?}", manual_request),
            format!("{:?}", delete_dataset)
        );
    }

    #[test]
    fn example_2() {
        let zosmf = get_zosmf();

        let manual_request = zosmf
            .core
            .client
            .delete("https://test.com/zosmf/restfiles/ds/-(ZMF046)/JIAHJ.REST.TEST.DATASET2")
            .build()
            .unwrap();

        let delete_uncataloged = zosmf
            .datasets()
            .delete("JIAHJ.REST.TEST.DATASET2")
            .volume("ZMF046")
            .get_request()
            .unwrap();

        assert_eq!(
            format!("{:?}", manual_request),
            format!("{:?}", delete_uncataloged)
        );
    }

    #[test]
    fn example_3() {
        let zosmf = get_zosmf();

        let manual_request = zosmf
            .core
            .client
            .delete("https://test.com/zosmf/restfiles/ds/JIAHJ.REST.TEST.PDS(MEMBER01)")
            .build()
            .unwrap();

        let delete_member = zosmf
            .datasets()
            .delete("JIAHJ.REST.TEST.PDS")
            .member("MEMBER01")
            .get_request()
            .unwrap();

        assert_eq!(
            format!("{:?}", manual_request),
            format!("{:?}", delete_member)
        );
    }

    #[test]
    fn example_4() {
        let zosmf = get_zosmf();

        let manual_request = zosmf
            .core
            .client
            .delete(
                "https://test.com/zosmf/restfiles/ds/-(ZMF046)/JIAHJ.REST.TEST.PDS.UNCAT(MEMBER01)",
            )
            .build()
            .unwrap();

        let delete_uncataloged_member = zosmf
            .datasets()
            .delete("JIAHJ.REST.TEST.PDS.UNCAT")
            .member("MEMBER01")
            .volume("ZMF046")
            .get_request()
            .unwrap();

        assert_eq!(
            format!("{:?}", manual_request),
            format!("{:?}", delete_uncataloged_member)
        );
    }
}
