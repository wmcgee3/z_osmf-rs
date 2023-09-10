pub mod list;
pub mod list_members;

use reqwest::Client;

use self::list::ListBuilder;
use self::list_members::{ListMembersBuilder, Member};

#[derive(Clone, Debug)]
pub struct Datasets<'a> {
    base_url: &'a str,
    client: &'a Client,
}

impl<'a> Datasets<'a> {
    pub(super) fn new(base_url: &'a str, client: &'a Client) -> Self {
        Datasets { base_url, client }
    }

    pub fn list(&self, name_pattern: &str) -> ListBuilder<'a> {
        ListBuilder::new(self.base_url, self.client, name_pattern)
    }

    pub fn list_members(&self, dataset_name: &str) -> ListMembersBuilder<'a, Member> {
        ListMembersBuilder::new(self.base_url, self.client, dataset_name.to_string())
    }
}
