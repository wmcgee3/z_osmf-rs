pub mod create;
pub mod delete;
pub mod list;
pub mod read;
pub mod write;

pub use create::*;
pub use delete::*;
pub use list::*;
pub use read::*;
pub use write::*;

use std::sync::Arc;

use z_osmf_core::restfiles::data_type::Text;

#[derive(Clone, Debug)]
pub struct FilesClient {
    base_url: Arc<str>,
    client: reqwest::Client,
}

impl FilesClient {
    pub(crate) fn new(base_url: Arc<str>, client: reqwest::Client) -> Self {
        FilesClient { base_url, client }
    }

    pub fn create(&self, path: &str) -> FileCreateBuilder {
        FileCreateBuilder::new(self.base_url.clone(), self.client.clone(), path)
    }

    pub fn delete(&self, path: &str) -> FileDeleteBuilder {
        FileDeleteBuilder::new(self.base_url.clone(), self.client.clone(), path)
    }

    pub fn list(&self, path: &str) -> FileListBuilder {
        FileListBuilder::new(self.base_url.clone(), self.client.clone(), path)
    }

    pub fn read(&self, path: &str) -> FileReadBuilder<Text> {
        FileReadBuilder::new(self.base_url.clone(), self.client.clone(), path)
    }

    pub fn write(&self, path: &str) -> FileWriteBuilder<String, Text> {
        FileWriteBuilder::new(self.base_url.clone(), self.client.clone(), path)
    }
}
