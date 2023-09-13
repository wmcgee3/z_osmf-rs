pub mod list;
pub mod list_members;
pub mod read;

pub use self::list::*;
pub use self::list_members::*;
pub use self::read::*;

use reqwest::Client;

mod utils;

#[derive(Clone, Debug)]
pub struct Datasets<'a> {
    base_url: &'a str,
    client: &'a Client,
}

impl<'a> Datasets<'a> {
    pub(super) fn new(base_url: &'a str, client: &'a Client) -> Self {
        Datasets { base_url, client }
    }

    pub fn list(&self, name_pattern: &str) -> DatasetListBuilder<'a, DatasetName> {
        DatasetListBuilder::new(self.base_url, self.client, name_pattern)
    }

    pub fn list_members(&self, dataset_name: &str) -> MemberListBuilder<'a, MemberName> {
        MemberListBuilder::new(self.base_url, self.client, dataset_name.to_string())
    }
}
