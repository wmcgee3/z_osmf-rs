pub mod copy;
pub mod copy_file;
pub mod create;
pub mod delete;
pub mod list;
pub mod members;
pub mod migrate;
pub mod read;
pub mod recall;
pub mod rename;
pub mod write;

use std::sync::Arc;

use reqwest::header::HeaderValue;
use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::ClientCore;

use self::copy::CopyBuilder;
use self::copy_file::CopyFileBuilder;
use self::create::CreateBuilder;
use self::delete::DeleteBuilder;
use self::list::{DatasetName, Datasets, DatasetsBuilder};
use self::members::{MemberName, Members, MembersBuilder};
use self::migrate::{Migrate, MigrateBuilder};
use self::read::{Read, ReadBuilder};
use self::recall::RecallBuilder;
use self::rename::RenameBuilder;
use self::write::{Write, WriteBuilder};

#[derive(Clone, Debug)]
pub struct DatasetsClient {
    core: Arc<ClientCore>,
}

/// # Datasets
impl DatasetsClient {
    pub(crate) fn new(core: &Arc<ClientCore>) -> Self {
        DatasetsClient { core: core.clone() }
    }

    /// #Examples
    ///
    /// Copy a dataset:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let copy_dataset = zosmf
    ///     .datasets()
    ///     .copy("MY.OLD.DS", "MY.NEW.DS")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Copy a PDS member:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let copy_dataset = zosmf
    ///     .datasets()
    ///     .copy("MY.OLD.PDS", "MY.NEW.PDS")
    ///     .from_member("OLD")
    ///     .to_member("NEW")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn copy(&self, from_dataset: &str, to_dataset: &str) -> CopyBuilder<String> {
        CopyBuilder::new(self.core.clone(), from_dataset, to_dataset)
    }

    /// #Examples
    ///
    /// Copy a file to a dataset:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let copy_dataset = zosmf
    ///     .datasets()
    ///     .copy_file("/u/jiahj/text.txt", "MY.NEW.DS")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Copy a file to a PDS member:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let copy_dataset = zosmf
    ///     .datasets()
    ///     .copy_file("/u/jiahj/text.txt", "MY.NEW.PDS")
    ///     .to_member("TEXT")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn copy_file(&self, from_path: &str, to_dataset: &str) -> CopyFileBuilder<String> {
        CopyFileBuilder::new(self.core.clone(), from_path, to_dataset)
    }

    /// # Examples
    ///
    /// Creating a sequential dataset:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let create_dataset = zosmf
    ///     .datasets()
    ///     .create("JIAHJ.REST.TEST.NEWDS")
    ///     .volume("zmf046")
    ///     .device_type("3390")
    ///     .organization("PS")
    ///     .space_allocation_unit("TRK")
    ///     .primary_space(10)
    ///     .secondary_space(5)
    ///     .average_block_size(500)
    ///     .record_format("FB")
    ///     .block_size(400)
    ///     .record_length(80)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Creating a partitioned dataset (PDS):
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let create_pds = zosmf
    ///     .datasets()
    ///     .create("JIAHJ.REST.TEST.NEWDS02")
    ///     .volume("zmf046")
    ///     .device_type("3390")
    ///     .organization("PO")
    ///     .space_allocation_unit("TRK")
    ///     .primary_space(10)
    ///     .secondary_space(5)
    ///     .directory_blocks(10)
    ///     .average_block_size(500)
    ///     .record_format("FB")
    ///     .block_size(400)
    ///     .record_length(80)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Creating a library / partitioned dataset extended (PDS-E):
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let create_pdse = zosmf
    ///     .datasets()
    ///     .create("JIAHJ.REST.TEST.NEWDS02")
    ///     .volume("zmf046")
    ///     .device_type("3390")
    ///     .organization("PO")
    ///     .space_allocation_unit("TRK")
    ///     .primary_space(10)
    ///     .secondary_space(5)
    ///     .directory_blocks(10)
    ///     .average_block_size(500)
    ///     .record_format("FB")
    ///     .block_size(400)
    ///     .record_length(80)
    ///     .dataset_type("LIBRARY")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create(&self, dataset_name: &str) -> CreateBuilder<String> {
        CreateBuilder::new(self.core.clone(), dataset_name)
    }

    /// # Examples
    ///
    /// Deleting a sequential dataset:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let delete_dataset = zosmf
    ///     .datasets()
    ///     .delete("JIAHJ.REST.TEST.DATASET")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Deleting an uncataloged sequential dataset:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let delete_uncataloged = zosmf
    ///     .datasets()
    ///     .delete("JIAHJ.REST.TEST.DATASET2")
    ///     .volume("ZMF046")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Deleting a PDS member:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let delete_member = zosmf
    ///     .datasets()
    ///     .delete("JIAHJ.REST.TEST.PDS")
    ///     .member("MEMBER01")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Deleting an uncataloged PDS member:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let delete_uncataloged_member = zosmf
    ///     .datasets()
    ///     .delete("JIAHJ.REST.TEST.PDS.UNCAT")
    ///     .member("MEMBER01")
    ///     .volume("ZMF046")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn delete(&self, dataset_name: &str) -> DeleteBuilder<String> {
        DeleteBuilder::new(self.core.clone(), dataset_name)
    }

    /// # Examples
    ///
    /// Listing datasets:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let list_datasets = zosmf
    ///     .datasets()
    ///     .list("IBMUSER.CONFIG.*")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Listing the base attributes of uncataloged datasets:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let list_datasets_base = zosmf
    ///     .datasets()
    ///     .list("**")
    ///     .volume("PEVTS2")
    ///     .attributes_base()
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(&self, name_pattern: &str) -> DatasetsBuilder<Datasets<DatasetName>> {
        DatasetsBuilder::new(self.core.clone(), name_pattern)
    }

    /// # Examples
    ///
    /// Listing PDS members:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let list_members = zosmf
    ///     .datasets()
    ///     .members("NOTSYS1.PROCLIB")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Listing the base attributes of PDS members:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let list_members_base = zosmf
    ///     .datasets()
    ///     .members("NOTSYS1.PROCLIB")
    ///     .attributes_base()
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn members(&self, dataset_name: &str) -> MembersBuilder<Members<MemberName>> {
        MembersBuilder::new(self.core.clone(), dataset_name)
    }

    /// # Examples
    ///
    /// Migrate a dataset:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let migrate_dataset = zosmf
    ///     .datasets()
    ///     .migrate("MY.TEST.DS")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn migrate(&self, name: &str) -> MigrateBuilder<Migrate> {
        MigrateBuilder::new(self.core.clone(), name)
    }

    /// # Examples
    ///
    /// Reading a PDS member:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let read_member = zosmf
    ///     .datasets()
    ///     .read("SYS1.PARMLIB")
    ///     .member("SMFPRM00")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Reading a sequential dataset:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let read_dataset = zosmf
    ///     .datasets()
    ///     .read("JIAHJ.REST.SRVMP")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn read<N>(&self, dataset_name: &N) -> ReadBuilder<Read<Box<str>>>
    where
        N: ToString + ?Sized,
    {
        ReadBuilder::new(self.core.clone(), dataset_name.to_string())
    }

    /// # Examples
    ///
    /// Recall a dataset:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let recall_dataset = zosmf
    ///     .datasets()
    ///     .recall("MY.MIGR.DS")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn recall(&self, name: &str) -> RecallBuilder<String> {
        RecallBuilder::new(self.core.clone(), name)
    }

    /// # Examples
    ///
    /// Renaming MY.OLD.DSN to MY.NEW.DSN:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let rename_dataset = zosmf
    ///     .datasets()
    ///     .rename("MY.OLD.DSN", "MY.NEW.DSN")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn rename(&self, from_dataset: &str, to_dataset: &str) -> RenameBuilder<String> {
        RenameBuilder::new(self.core.clone(), from_dataset, to_dataset)
    }

    /// # Examples
    ///
    /// Writing to a PDS member:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// # let string_data = "".to_string();
    /// let write_dataset = zosmf
    ///     .datasets()
    ///     .write("SYS1.PARMLIB")
    ///     .member("SMFPRM00")
    ///     .if_match("B5C6454F783590AA8EC15BD88E29EA63")
    ///     .text(string_data)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn write(&self, dataset_name: &str) -> WriteBuilder<Write> {
        WriteBuilder::new(self.core.clone(), dataset_name)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum DatasetDataType {
    Binary,
    Record,
    Text,
}

impl std::fmt::Display for DatasetDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DatasetDataType::Binary => "binary",
                DatasetDataType::Record => "record",
                DatasetDataType::Text => "text",
            }
        )
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MigratedRecall {
    Error,
    NoWait,
    Wait,
}

impl From<MigratedRecall> for HeaderValue {
    fn from(val: MigratedRecall) -> HeaderValue {
        match val {
            MigratedRecall::Error => "error",
            MigratedRecall::NoWait => "nowait",
            MigratedRecall::Wait => "wait",
        }
        .try_into()
        .unwrap()
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize)]
pub enum Enqueue {
    #[serde(rename = "EXCLU")]
    Exclusive,
    #[serde(rename = "SHRW")]
    SharedReadWrite,
}

impl From<Enqueue> for HeaderValue {
    fn from(val: Enqueue) -> HeaderValue {
        match val {
            Enqueue::Exclusive => "EXCLU",
            Enqueue::SharedReadWrite => "SHRW",
        }
        .try_into()
        .unwrap()
    }
}

fn get_member(value: &Option<Box<str>>) -> String {
    value
        .as_ref()
        .map(|v| format!("({})", v))
        .unwrap_or("".to_string())
}

fn get_session_ref(response: &reqwest::Response) -> Result<Option<Box<str>>, Error> {
    Ok(response
        .headers()
        .get("X-IBM-Session-Ref")
        .map(|v| v.to_str())
        .transpose()?
        .map(|v| v.into()))
}

fn get_volume(value: &Option<Box<str>>) -> String {
    value
        .as_ref()
        .map(|v| format!("/-({})", v))
        .unwrap_or("".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_data_type() {
        assert_eq!(format!("{}", DatasetDataType::Binary), "binary");

        assert_eq!(format!("{}", DatasetDataType::Record), "record");

        assert_eq!(format!("{}", DatasetDataType::Text), "text");
    }

    #[test]
    fn display_migrated_recall() {
        let header_value: HeaderValue = MigratedRecall::Error.into();
        assert_eq!(header_value, HeaderValue::from_static("error"));

        let header_value: HeaderValue = MigratedRecall::NoWait.into();
        assert_eq!(header_value, HeaderValue::from_static("nowait"));

        let header_value: HeaderValue = MigratedRecall::Wait.into();
        assert_eq!(header_value, HeaderValue::from_static("wait"));
    }

    #[test]
    fn display_obtain_enq() {
        let header_value: HeaderValue = Enqueue::Exclusive.into();
        assert_eq!(header_value, HeaderValue::from_static("EXCLU"));

        let header_value: HeaderValue = Enqueue::SharedReadWrite.into();
        assert_eq!(header_value, HeaderValue::from_static("SHRW"));
    }

    #[test]
    fn test_get_session_ref() {
        let response = reqwest::Response::from(
            http::Response::builder()
                .header("X-IBM-Session-Ref", "ABCD1234")
                .body("")
                .unwrap(),
        );
        assert_eq!(get_session_ref(&response).unwrap(), Some("ABCD1234".into()));

        let response = reqwest::Response::from(http::Response::new(""));
        assert_eq!(get_session_ref(&response).unwrap(), None);
    }
}
