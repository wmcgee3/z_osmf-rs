pub mod delete;
pub mod list;
pub mod read;

pub use delete::*;
pub use list::*;
pub use read::*;

use reqwest::Client;

use crate::data_type::Text;

#[derive(Clone, Debug)]
pub struct FilesClient<'a> {
    base_url: &'a str,
    client: &'a Client,
}

impl<'a> FilesClient<'a> {
    pub(crate) fn new(base_url: &'a str, client: &'a Client) -> Self {
        FilesClient { base_url, client }
    }

    pub fn list(&self, path: &str) -> FileListBuilder {
        FileListBuilder::new(self.base_url, self.client, path)
    }

    pub fn read(&self, file_path: &str) -> FileReadBuilder<Text> {
        FileReadBuilder::new(self.base_url, self.client, file_path)
    }

    pub fn delete(&self, file_path: &str) -> FileDeleteBuilder {
        FileDeleteBuilder::new(self.base_url, self.client, file_path)
    }
}
