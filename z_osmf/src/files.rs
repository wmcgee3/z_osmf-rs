pub mod copy;
pub mod copy_dataset;
pub mod create;
pub mod delete;
pub mod extra_attributes;
pub mod link;
pub mod list;
pub mod mode;
pub mod owner;
pub mod read;
pub mod rename;
pub mod tags;
pub mod unlink;
pub mod write;

use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::restfiles::Etag;
use crate::{ClientCore, Result};

use self::copy::FileCopyBuilder;
use self::copy_dataset::FileCopyDatasetBuilder;
use self::create::FileCreateBuilder;
use self::delete::FileDeleteBuilder;
use self::extra_attributes::reset::FileExtraAttributesResetBuilder;
use self::extra_attributes::set::FileExtraAttributesSetBuilder;
use self::extra_attributes::{FileExtraAttributeList, FileExtraAttributeListBuilder};
use self::link::{FileLinkBuilder, FileLinkType};
use self::list::{FileList, FileListBuilder};
use self::mode::FileChangeModeBuilder;
use self::owner::FileChangeOwnerBuilder;
use self::read::{FileRead, FileReadBuilder};
use self::rename::FileRenameBuilder;
use self::tags::remove::FileTagsRemoveBuilder;
use self::tags::set::FileTagsSetBuilder;
use self::tags::{FileTagList, FileTagListBuilder};
use self::unlink::FileUnlinkBuilder;
use self::write::FileWriteBuilder;

#[derive(Clone, Debug)]
pub struct FilesClient {
    core: ClientCore,
}

/// # Files
impl FilesClient {
    pub(crate) fn new(core: ClientCore) -> Self {
        FilesClient { core }
    }

    /// # Examples
    ///
    /// Change the mode (permissions) of a file:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let change_mode = zosmf
    ///     .files()
    ///     .change_mode("/u/jiahj/test.txt", "755")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Change the mode (permissions) of a directory and the files within:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let change_mode = zosmf
    ///     .files()
    ///     .change_mode("/u/jiahj/testDir", "755")
    ///     .recursive(true)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn change_mode<P, M>(&self, path: P, mode: M) -> FileChangeModeBuilder<String>
    where
        P: std::fmt::Display,
        M: std::fmt::Display,
    {
        FileChangeModeBuilder::new(self.core.clone(), path, mode)
    }

    /// # Examples
    ///
    /// Change the owner of a file:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let change_owner = zosmf
    ///     .files()
    ///     .change_owner("/u/jiahj/test.txt", "ibmuser")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Change the owner of a directory and the files within:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let change_owner = zosmf
    ///     .files()
    ///     .change_owner("/u/jiahj/testDir", "ibmuser")
    ///     .recursive(true)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Change the owning user and group:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let change_owner = zosmf
    ///     .files()
    ///     .change_owner("/u/jiahj/test.txt", "ibmuser")
    ///     .group("ibmgrp")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn change_owner<P, O>(&self, path: P, owner: O) -> FileChangeOwnerBuilder<String>
    where
        P: std::fmt::Display,
        O: std::fmt::Display,
    {
        FileChangeOwnerBuilder::new(self.core.clone(), path, owner)
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
    pub fn copy<F, T>(&self, from_path: F, to_path: T) -> FileCopyBuilder<String>
    where
        F: std::fmt::Display,
        T: std::fmt::Display,
    {
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
    pub fn copy_dataset<F, T>(&self, from_dataset: F, to_path: T) -> FileCopyDatasetBuilder<String>
    where
        F: std::fmt::Display,
        T: std::fmt::Display,
    {
        FileCopyDatasetBuilder::new(self.core.clone(), from_dataset, to_path)
    }

    /// # Examples
    ///
    /// Create a file:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// # use z_osmf::files::create::FileCreateType;
    /// let create_file = zosmf
    ///     .files()
    ///     .create("/u/jiahj/text.txt")
    ///     .file_type(FileCreateType::File)
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
    /// # use z_osmf::files::create::FileCreateType;
    /// let create_file = zosmf
    ///     .files()
    ///     .create("/u/jiahj/testDir")
    ///     .file_type(FileCreateType::Directory)
    ///     .mode("rwxr-xrwx")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create<P>(&self, path: P) -> FileCreateBuilder<String>
    where
        P: std::fmt::Display,
    {
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
    pub fn delete<P>(&self, path: P) -> FileDeleteBuilder<String>
    where
        P: std::fmt::Display,
    {
        FileDeleteBuilder::new(self.core.clone(), path)
    }

    /// # Examples
    ///
    /// Get the extra attributes of a file:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let file_extra_attributes = zosmf
    ///     .files()
    ///     .get_extra_attributes("/u/jiahj/testFile.txt")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_extra_attributes<P>(&self, path: P) -> Result<FileExtraAttributeList>
    where
        P: std::fmt::Display,
    {
        FileExtraAttributeListBuilder::new(self.core.clone(), path)
            .build()
            .await
    }

    /// # Examples
    ///
    /// Link a file or directory:
    /// ```
    /// # use z_osmf::files::link::FileLinkType;
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let file_link = zosmf
    ///     .files()
    ///     .link(FileLinkType::Symbol, "/u/jiahj/sourceFile.txt", "/u/jiahj/targetFile.txt")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn link<S, T>(
        &self,
        link_type: FileLinkType,
        source_path: S,
        target_path: T,
    ) -> FileLinkBuilder<String>
    where
        S: std::fmt::Display,
        T: std::fmt::Display,
    {
        FileLinkBuilder::new(self.core.clone(), source_path, target_path, link_type)
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
    pub fn list<P>(&self, path: P) -> FileListBuilder<FileList>
    where
        P: std::fmt::Display,
    {
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
    pub fn list_tag<P>(&self, path: P) -> FileTagListBuilder<FileTagList>
    where
        P: std::fmt::Display,
    {
        FileTagListBuilder::new(self.core.clone(), path)
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
    pub fn read<P>(&self, path: P) -> FileReadBuilder<FileRead<Arc<str>>>
    where
        P: std::fmt::Display,
    {
        FileReadBuilder::new(self.core.clone(), path)
    }

    /// # Examples
    ///
    /// Remove the tag on a file:
    /// ```
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
    pub fn remove_tag<P>(&self, path: P) -> FileTagsRemoveBuilder<String>
    where
        P: std::fmt::Display,
    {
        FileTagsRemoveBuilder::new(self.core.clone(), path)
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
    pub fn rename<F, T>(&self, from_path: F, to_path: T) -> FileRenameBuilder<String>
    where
        F: std::fmt::Display,
        T: std::fmt::Display,
    {
        FileRenameBuilder::new(self.core.clone(), from_path, to_path)
    }

    /// # Examples
    ///
    /// Remove extra attributes from a file:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let file_reset_extra_attributes = zosmf
    ///     .files()
    ///     .reset_extra_attributes("/u/jiahj/testFile.txt")
    ///     .apf_authorized(true)
    ///     .shared_library(true)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn reset_extra_attributes<P>(&self, path: P) -> FileExtraAttributesResetBuilder<String>
    where
        P: std::fmt::Display,
    {
        FileExtraAttributesResetBuilder::new(self.core.clone(), path)
    }

    /// # Examples
    ///
    /// Add extra attributes to a file:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let file_set_extra_attributes = zosmf
    ///     .files()
    ///     .set_extra_attributes("/u/jiahj/testFile.txt")
    ///     .program_controlled(true)
    ///     .shared_address_space(true)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_extra_attributes<P>(&self, path: P) -> FileExtraAttributesSetBuilder<String>
    where
        P: std::fmt::Display,
    {
        FileExtraAttributesSetBuilder::new(self.core.clone(), path)
    }

    /// # Examples
    ///
    /// Set the tag on a file:
    /// ```
    /// # use z_osmf::files::tags::FileTagType;
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
    /// # use z_osmf::files::tags::FileTagType;
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
    pub fn set_tag<P>(&self, path: P) -> FileTagsSetBuilder<String>
    where
        P: std::fmt::Display,
    {
        FileTagsSetBuilder::new(self.core.clone(), path)
    }

    /// # Examples
    ///
    /// Unlink a file or directory:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let file_unlink = zosmf
    ///     .files()
    ///     .unlink("/u/jiahj/targetFile.txt")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn unlink<P>(&self, path: P) -> Result<String>
    where
        P: std::fmt::Display,
    {
        FileUnlinkBuilder::new(self.core.clone(), path)
            .build()
            .await
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
    pub fn write<P>(&self, path: P) -> FileWriteBuilder<Etag>
    where
        P: std::fmt::Display,
    {
        FileWriteBuilder::new(self.core.clone(), path)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum FileDataType {
    Binary,
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

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum FileTagType {
    Binary,
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
