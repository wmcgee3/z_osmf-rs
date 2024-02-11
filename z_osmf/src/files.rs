pub mod copy;
pub mod copy_dataset;
pub mod create;
pub mod delete;
pub mod list;
pub mod list_tag;
pub mod read;
pub mod remove_tag;
pub mod rename;
pub mod set_tag;
pub mod write;

use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::ClientCore;

use self::copy::{FileCopy, FileCopyBuilder};
use self::copy_dataset::{FileCopyDataset, FileCopyDatasetBuilder};
use self::create::{FileCreate, FileCreateBuilder};
use self::delete::{FileDelete, FileDeleteBuilder};
use self::list::{FileList, FileListBuilder};
use self::list_tag::{FileListTag, FileListTagBuilder};
use self::read::{FileRead, FileReadBuilder};
use self::remove_tag::{FileRemoveTag, FileRemoveTagBuilder};
use self::rename::{FileRename, FileRenameBuilder};
use self::set_tag::{FileSetTag, FileSetTagBuilder};
use self::write::{FileWrite, FileWriteBuilder};

#[derive(Clone, Debug)]
pub struct FilesClient {
    core: Arc<ClientCore>,
}

/// # Files
impl FilesClient {
    pub(crate) fn new(core: &Arc<ClientCore>) -> Self {
        FilesClient { core: core.clone() }
    }

    pub fn chmod(&self) {
        todo!()
    }

    pub fn chown(&self) {
        todo!()
    }

    /// # Examples
    ///
    /// Copy a file:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let file_copy = zosmf
    ///     .files()
    ///     .copy("/u/jiahj/test.txt", "/u/jiahj/test2.txt")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Copy a file and overwrite the target, if it exists:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let file_copy = zosmf
    ///     .files()
    ///     .copy("/u/jiahj/test.txt", "/u/jiahj/test2.txt")
    ///     .overwrite(true)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn copy(&self, from_path: &str, to_path: &str) -> FileCopyBuilder<FileCopy> {
        FileCopyBuilder::new(self.core.clone(), from_path, to_path)
    }

    /// # Examples
    ///
    /// Copy a dataset to a file:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let file_copy_dataset = zosmf
    ///     .files()
    ///     .copy_dataset("MY.SRC.DS", "/u/jiahj/test2.txt")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Copy a PDS member to a file:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let file_copy_dataset = zosmf
    ///     .files()
    ///     .copy_dataset("MY.SRC.PDS", "/u/jiahj/test2.txt")
    ///     .from_member("TEST")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn copy_dataset(
        &self,
        from_dataset: &str,
        to_path: &str,
    ) -> FileCopyDatasetBuilder<FileCopyDataset> {
        FileCopyDatasetBuilder::new(self.core.clone(), from_dataset, to_path)
    }

    /// # Examples
    ///
    /// Create a file:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// # use z_osmf::files::create::CreateFileType;
    /// let create_file = zosmf
    ///     .files()
    ///     .create("/u/jiahj/text.txt")
    ///     .file_type(CreateFileType::File)
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
    /// # use z_osmf::files::create::CreateFileType;
    /// let create_file = zosmf
    ///     .files()
    ///     .create("/u/jiahj/testDir")
    ///     .file_type(CreateFileType::Directory)
    ///     .mode("rwxr-xrwx")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create(&self, path: &str) -> FileCreateBuilder<FileCreate> {
        FileCreateBuilder::new(self.core.clone(), path)
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
        FileDeleteBuilder::new(self.core.clone(), path)
    }

    pub fn extattr(&self) {
        todo!()
    }

    pub fn getfacl(&self) {
        todo!()
    }

    pub fn link(&self) {
        todo!()
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
        FileListBuilder::new(self.core.clone(), path)
    }

    /// # Examples
    ///
    /// List the tag of a file:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let list_tag = zosmf
    ///     .files()
    ///     .list_tag("/u/jiahj/text.txt")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// List the tags of files in a directory:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let list_tag = zosmf
    ///     .files()
    ///     .list_tag("/u/jiahj/testDir")
    ///     .recursive(true)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_tag(&self, path: &str) -> FileListTagBuilder<FileListTag> {
        FileListTagBuilder::new(self.core.clone(), path)
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
        FileReadBuilder::new(self.core.clone(), path)
    }

    /// # Examples
    ///
    /// Remove the tag on a file:
    /// ```
    /// # use z_osmf::files::FileTagType;
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let remove_tag = zosmf
    ///     .files()
    ///     .remove_tag("/u/jiahj/test.txt")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Remove the tag on all files in a directory:
    /// ```
    /// # use z_osmf::files::FileTagType;
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let remove_tag = zosmf
    ///     .files()
    ///     .remove_tag("/u/jiahj/testDir")
    ///     .recursive(true)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn remove_tag(&self, path: &str) -> FileRemoveTagBuilder<FileRemoveTag> {
        FileRemoveTagBuilder::new(self.core.clone(), path)
    }

    /// # Examples
    ///
    /// Rename (move) a file:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let file_rename = zosmf
    ///     .files()
    ///     .rename("/u/jiahj/test.txt", "/u/jiahj/test2.txt")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Rename (move) a file and overwrite the target, if it exists:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let file_rename = zosmf
    ///     .files()
    ///     .rename("/u/jiahj/test.txt", "/u/jiahj/test2.txt")
    ///     .overwrite(true)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn rename(&self, from_path: &str, to_path: &str) -> FileRenameBuilder<FileRename> {
        FileRenameBuilder::new(self.core.clone(), from_path, to_path)
    }

    /// # Examples
    ///
    /// Set the tag on a file:
    /// ```
    /// # use z_osmf::files::FileTagType;
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let set_tag = zosmf
    ///     .files()
    ///     .set_tag("/u/jiahj/test.txt")
    ///     .tag_type(FileTagType::Text)
    ///     .code_set("IBM-1047")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Set the tag on all files in a directory:
    /// ```
    /// # use z_osmf::files::FileTagType;
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let set_tag = zosmf
    ///     .files()
    ///     .set_tag("/u/jiahj/testDir")
    ///     .tag_type(FileTagType::Text)
    ///     .code_set("IBM-1047")
    ///     .recursive(true)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_tag(&self, path: &str) -> FileSetTagBuilder<FileSetTag> {
        FileSetTagBuilder::new(self.core.clone(), path)
    }

    pub fn setfacl(&self) {
        todo!()
    }

    pub fn unlink(&self) {
        todo!()
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
        FileWriteBuilder::new(self.core.clone(), path)
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum FileDataType {
    Binary,
    #[default]
    Text,
}

impl std::fmt::Display for FileDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FileDataType::Binary => "binary",
                FileDataType::Text => "text",
            }
        )
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum FileTagLinks {
    #[default]
    Change,
    Suppress,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum FileTagType {
    Binary,
    #[default]
    Mixed,
    Text,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data_type_display() {
        assert_eq!(format!("{}", FileDataType::Binary), "binary");

        assert_eq!(format!("{}", FileDataType::Text), "text");
    }
}
