pub mod create;
pub mod delete;
pub mod list;
pub mod read;
pub mod write;

pub use self::create::*;
pub use self::delete::*;
pub use self::list::*;
pub use self::read::*;
pub use self::write::*;

use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct FilesClient {
    base_url: Arc<str>,
    client: reqwest::Client,
}

impl FilesClient {
    pub(crate) fn new(base_url: Arc<str>, client: reqwest::Client) -> Self {
        FilesClient { base_url, client }
    }

    /// # Examples
    ///
    /// Create a file:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// # use z_osmf::files::FileType;
    /// let create_file = zosmf
    ///     .files()
    ///     .create("/u/jiahj/text.txt")
    ///     .file_type(FileType::File)
    ///     .mode("RWXRW-RW-")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Create a directory:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// # use z_osmf::files::FileType;
    /// let create_file = zosmf
    ///     .files()
    ///     .create("/u/jiahj/testDir")
    ///     .file_type(FileType::Directory)
    ///     .mode("rwxr-xrwx")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create(&self, path: &str) -> FileCreateBuilder<FileCreate> {
        FileCreateBuilder::new(self.base_url.clone(), self.client.clone(), path)
    }

    /// # Examples
    ///
    /// Delete a file:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let delete_file = zosmf
    ///     .files()
    ///     .delete("/u/jiahj/text.txt")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Delete a directory:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let delete_file = zosmf
    ///     .files()
    ///     .delete("/u/jiahj/testDir")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn delete(&self, path: &str) -> FileDeleteBuilder<FileDelete> {
        FileDeleteBuilder::new(self.base_url.clone(), self.client.clone(), path)
    }

    /// # Examples
    ///
    /// List files and directories:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let list_files = zosmf
    ///     .files()
    ///     .list("/usr")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// List a single file:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let list_files = zosmf
    ///     .files()
    ///     .list("/u/ibmuser/myFile.txt")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// List files filtering by name:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let list_files = zosmf
    ///     .files()
    ///     .list("/usr/include")
    ///     .name("f*.h")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(&self, path: &str) -> FileListBuilder<FileList> {
        FileListBuilder::new(self.base_url.clone(), self.client.clone(), path)
    }

    /// # Examples
    ///
    /// Read a file:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let read_file = zosmf
    ///     .files()
    ///     .read("/etc/inetd.conf")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn read(&self, path: &str) -> FileReadBuilder<FileRead<Box<str>>> {
        FileReadBuilder::new(self.base_url.clone(), self.client.clone(), path)
    }

    /// # Examples
    ///
    /// Write to a file:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// # let text_data = "";
    /// let write_file = zosmf
    ///     .files()
    ///     .write("/etc/inetd.conf")
    ///     .text(text_data)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn write(&self, path: &str) -> FileWriteBuilder<FileWrite> {
        FileWriteBuilder::new(self.base_url.clone(), self.client.clone(), path)
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum DataType {
    Binary,
    #[default]
    Text,
}

impl std::fmt::Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DataType::Binary => "binary",
                DataType::Text => "text",
            }
        )
    }
}
