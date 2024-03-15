pub mod mode;
pub mod owner;
pub mod copy;
pub mod copy_dataset;
pub mod create;
pub mod delete;
pub mod extra_attributes;
pub mod link;
pub mod list;
pub mod read;
pub mod rename;
pub mod tags;
pub mod unlink;
pub mod write;

use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::{ClientCore, TransactionId};

use self::mode::ChangeModeBuilder;
use self::owner::ChangeOwnerBuilder;
use self::copy::CopyBuilder;
use self::copy_dataset::CopyDatasetBuilder;
use self::create::CreateBuilder;
use self::delete::DeleteBuilder;
use self::extra_attributes::{
    GetExtraAttributes, ResetExtraAttributesBuilder, SetExtraAttributesBuilder,
};
use self::link::{LinkBuilder, LinkType};
use self::list::{Files, FilesBuilder};
use self::read::{Read, ReadBuilder};
use self::rename::RenameBuilder;
use self::tags::{RemoveBuilder, SetBuilder, Tags, TagsBuilder};
use self::unlink::UnlinkBuilder;
use self::write::{Write, WriteBuilder};

#[derive(Clone, Debug)]
pub struct FilesClient {
    core: Arc<ClientCore>,
}

/// # Files
impl FilesClient {
    pub(crate) fn new(core: &Arc<ClientCore>) -> Self {
        FilesClient { core: core.clone() }
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
    pub fn change_mode(&self, path: &str, mode: &str) -> ChangeModeBuilder<TransactionId> {
        ChangeModeBuilder::new(self.core.clone(), path, mode)
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
    pub fn change_owner(&self, path: &str, owner: &str) -> ChangeOwnerBuilder<TransactionId> {
        ChangeOwnerBuilder::new(self.core.clone(), path, owner)
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
    pub fn copy(&self, from_path: &str, to_path: &str) -> CopyBuilder<TransactionId> {
        CopyBuilder::new(self.core.clone(), from_path, to_path)
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
    ) -> CopyDatasetBuilder<TransactionId> {
        CopyDatasetBuilder::new(self.core.clone(), from_dataset, to_path)
    }

    /// # Examples
    ///
    /// Create a file:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// # use z_osmf::files::create::FileType;
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
    /// # use z_osmf::files::create::FileType;
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
    pub fn create(&self, path: &str) -> CreateBuilder<TransactionId> {
        CreateBuilder::new(self.core.clone(), path)
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
    pub fn delete(&self, path: &str) -> DeleteBuilder<TransactionId> {
        DeleteBuilder::new(self.core.clone(), path)
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
    pub async fn get_extra_attributes(
        &self,
        path: &str,
    ) -> Result<GetExtraAttributes, crate::error::Error> {
        GetExtraAttributes::new(&self.core, path).await
    }

    /// # Examples
    ///
    /// Link a file or directory:
    /// ```
    /// # use z_osmf::files::link::LinkType;
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let file_link = zosmf
    ///     .files()
    ///     .link(LinkType::Symbol, "/u/jiahj/sourceFile.txt", "/u/jiahj/targetFile.txt")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn link(
        &self,
        link_type: LinkType,
        source_path: &str,
        target_path: &str,
    ) -> LinkBuilder<TransactionId> {
        LinkBuilder::new(self.core.clone(), source_path, target_path, link_type)
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
    pub fn list(&self, path: &str) -> FilesBuilder<Files> {
        FilesBuilder::new(self.core.clone(), path)
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
    pub fn list_tag(&self, path: &str) -> TagsBuilder<Tags> {
        TagsBuilder::new(self.core.clone(), path)
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
    pub fn read(&self, path: &str) -> ReadBuilder<Read<Box<str>>> {
        ReadBuilder::new(self.core.clone(), path)
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
    pub fn remove_tag(&self, path: &str) -> RemoveBuilder<TransactionId> {
        RemoveBuilder::new(self.core.clone(), path)
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
    pub fn rename(&self, from_path: &str, to_path: &str) -> RenameBuilder<TransactionId> {
        RenameBuilder::new(self.core.clone(), from_path, to_path)
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
    pub fn reset_extra_attributes(&self, path: &str) -> ResetExtraAttributesBuilder<TransactionId> {
        ResetExtraAttributesBuilder::new(self.core.clone(), path)
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
    pub fn set_extra_attributes(&self, path: &str) -> SetExtraAttributesBuilder<TransactionId> {
        SetExtraAttributesBuilder::new(self.core.clone(), path)
    }

    /// # Examples
    ///
    /// Set the tag on a file:
    /// ```
    /// # use z_osmf::files::tags::TagType;
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let set_tag = zosmf
    ///     .files()
    ///     .set_tag("/u/jiahj/test.txt")
    ///     .tag_type(TagType::Text)
    ///     .code_set("IBM-1047")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Set the tag on all files in a directory:
    /// ```
    /// # use z_osmf::files::tags::TagType;
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let set_tag = zosmf
    ///     .files()
    ///     .set_tag("/u/jiahj/testDir")
    ///     .tag_type(TagType::Text)
    ///     .code_set("IBM-1047")
    ///     .recursive(true)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_tag(&self, path: &str) -> SetBuilder<TransactionId> {
        SetBuilder::new(self.core.clone(), path)
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
    pub async fn unlink(&self, path: &str) -> Result<TransactionId, crate::error::Error> {
        UnlinkBuilder::new(self.core.clone(), path).build().await
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
    pub fn write(&self, path: &str) -> WriteBuilder<Write> {
        WriteBuilder::new(self.core.clone(), path)
    }
}

#[derive(
    Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize,
)]
#[serde(rename_all = "lowercase")]
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

#[derive(
    Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize,
)]
#[serde(rename_all = "lowercase")]
pub enum TagType {
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
        assert_eq!(format!("{}", DataType::Binary), "binary");

        assert_eq!(format!("{}", DataType::Text), "text");
    }
}
