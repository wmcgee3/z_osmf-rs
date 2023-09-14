pub mod read;

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

    pub fn read(&self, file_path: &str) -> FileReadBuilder<'a, Text> {
        FileReadBuilder::new(self.base_url, self.client, file_path)
    }
}
