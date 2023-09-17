pub mod delete;
pub mod list;
pub mod read;

pub use delete::*;
pub use list::*;
pub use read::*;

use std::sync::Arc;

use crate::data_type::Text;

#[derive(Clone, Debug)]
pub struct FilesClient {
    base_url: Arc<str>,
    client: reqwest::Client,
}

impl FilesClient {
    pub(crate) fn new(base_url: Arc<str>, client: reqwest::Client) -> Self {
        FilesClient { base_url, client }
    }

    pub fn list(&self, path: &str) -> FileListBuilder {
        FileListBuilder::new(self.base_url.clone(), self.client.clone(), path)
    }

    pub fn read(&self, file_path: &str) -> FileReadBuilder<Text> {
        FileReadBuilder::new(self.base_url.clone(), self.client.clone(), file_path)
    }

    pub fn delete(&self, file_path: &str) -> FileDeleteBuilder {
        FileDeleteBuilder::new(self.base_url.clone(), self.client.clone(), file_path)
    }
}
