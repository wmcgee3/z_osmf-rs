pub mod create;
pub mod delete;
pub mod list;
pub mod list_members;
pub mod read;
pub mod write;

pub use create::*;
pub use delete::*;
pub use list::*;
pub use list_members::*;
pub use read::*;
pub use write::*;

mod utils;

use std::sync::Arc;

use crate::data_type::Text;

#[derive(Clone, Debug)]
pub struct DatasetsClient {
    base_url: Arc<str>,
    client: reqwest::Client,
}

impl DatasetsClient {
    pub(super) fn new(base_url: Arc<str>, client: reqwest::Client) -> Self {
        DatasetsClient { base_url, client }
    }

    pub fn list(&self, name_pattern: &str) -> DatasetListBuilder<DatasetName> {
        DatasetListBuilder::new(self.base_url.clone(), self.client.clone(), name_pattern)
    }

    pub fn list_members(&self, dataset_name: &str) -> MemberListBuilder<MemberName> {
        MemberListBuilder::new(
            self.base_url.clone(),
            self.client.clone(),
            dataset_name.to_string(),
        )
    }

    pub fn read(&self, dataset_name: &str) -> DatasetReadBuilder<Text> {
        DatasetReadBuilder::new(self.base_url.clone(), self.client.clone(), dataset_name)
    }

    pub fn write(&self, dataset_name: &str) -> DatasetWriteBuilder<String, Text> {
        DatasetWriteBuilder::new(self.base_url.clone(), self.client.clone(), dataset_name)
    }

    pub fn create(&self, dataset_name: &str) -> DatasetCreateBuilder {
        DatasetCreateBuilder::new(self.base_url.clone(), self.client.clone(), dataset_name)
    }

    pub fn delete(&self, dataset_name: &str) -> DatasetDeleteBuilder {
        DatasetDeleteBuilder::new(self.base_url.clone(), self.client.clone(), dataset_name)
    }
}
