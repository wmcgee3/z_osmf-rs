pub mod delete;
pub mod list;
pub mod list_members;
pub mod read;
pub mod write;

pub use delete::*;
pub use list::*;
pub use list_members::*;
pub use read::*;
pub use write::*;

mod utils;

use reqwest::Client;

use crate::data_type::Text;

#[derive(Clone, Debug)]
pub struct DatasetsClient<'a> {
    base_url: &'a str,
    client: &'a Client,
}

impl<'a> DatasetsClient<'a> {
    pub(super) fn new(base_url: &'a str, client: &'a Client) -> Self {
        DatasetsClient { base_url, client }
    }

    pub fn list(&self, name_pattern: &str) -> DatasetListBuilder<'_, DatasetName> {
        DatasetListBuilder::new(self.base_url, self.client, name_pattern)
    }

    pub fn list_members(&self, dataset_name: &str) -> MemberListBuilder<'_, MemberName> {
        MemberListBuilder::new(self.base_url, self.client, dataset_name.to_string())
    }

    pub fn read(&self, dataset_name: &str) -> DatasetReadBuilder<'_, Text> {
        DatasetReadBuilder::new(self.base_url, self.client, dataset_name)
    }

    pub fn write(&self, dataset_name: &str) -> DatasetWriteBuilder<'_, String, Text> {
        DatasetWriteBuilder::new(self.base_url, self.client, dataset_name)
    }

    pub fn create(&self) {
        todo!()
    }

    pub fn delete(&self, dataset_name: &str) -> DatasetDeleteBuilder<'_> {
        DatasetDeleteBuilder::new(self.base_url, self.client, dataset_name)
    }
}
