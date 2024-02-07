pub mod copy;
pub mod copy_file;
pub mod create;
pub mod delete;
pub mod list;
pub mod member_list;
pub mod migrate;
pub mod read;
pub mod recall;
pub mod rename;
pub mod write;

use reqwest::header::HeaderValue;
use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::ZOsmf;

use self::copy::{DatasetCopy, DatasetCopyBuilder};
use self::copy_file::{CopyFileToDataset, CopyFileToDatasetBuilder};
use self::create::{DatasetCreate, DatasetCreateBuilder};
use self::delete::{DatasetDelete, DatasetDeleteBuilder};
use self::list::{DatasetList, DatasetListBuilder, DatasetName};
use self::member_list::{DatasetMemberList, DatasetMemberListBuilder, MemberName};
use self::migrate::{DatasetMigrate, DatasetMigrateBuilder};
use self::read::{DatasetRead, DatasetReadBuilder};
use self::recall::{DatasetRecall, DatasetRecallBuilder};
use self::rename::{DatasetRename, DatasetRenameBuilder};
use self::write::{DatasetWrite, DatasetWriteBuilder};

/// # Datasets
impl ZOsmf {
    pub fn copy_dataset(
        &self,
        from_dataset: &str,
        to_dataset: &str,
    ) -> DatasetCopyBuilder<DatasetCopy> {
        DatasetCopyBuilder::new(
            self.base_url.clone(),
            self.client.clone(),
            from_dataset,
            to_dataset,
        )
    }

    pub fn copy_file_to_dataset(
        &self,
        from_path: &str,
        to_dataset: &str,
    ) -> CopyFileToDatasetBuilder<CopyFileToDataset> {
        CopyFileToDatasetBuilder::new(
            self.base_url.clone(),
            self.client.clone(),
            from_path,
            to_dataset,
        )
    }

    /// # Examples
    ///
    /// Creating a sequential dataset:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let create_dataset = zosmf
    ///     .create_dataset("JIAHJ.REST.TEST.NEWDS")
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
    ///     .create_dataset("JIAHJ.REST.TEST.NEWDS02")
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
    ///     .create_dataset("JIAHJ.REST.TEST.NEWDS02")
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
    pub fn create_dataset(&self, dataset_name: &str) -> DatasetCreateBuilder<DatasetCreate> {
        DatasetCreateBuilder::new(self.base_url.clone(), self.client.clone(), dataset_name)
    }

    /// # Examples
    ///
    /// Deleting a sequential dataset:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let delete_dataset = zosmf
    ///     .delete_dataset("JIAHJ.REST.TEST.DATASET")
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
    ///     .delete_dataset("JIAHJ.REST.TEST.DATASET2")
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
    ///     .delete_dataset("JIAHJ.REST.TEST.PDS")
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
    ///     .delete_dataset("JIAHJ.REST.TEST.PDS.UNCAT")
    ///     .member("MEMBER01")
    ///     .volume("ZMF046")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn delete_dataset(&self, dataset_name: &str) -> DatasetDeleteBuilder<DatasetDelete> {
        DatasetDeleteBuilder::new(self.base_url.clone(), self.client.clone(), dataset_name)
    }

    /// # Examples
    ///
    /// Listing PDS members:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let list_members = zosmf
    ///     .list_dataset_members("SYS1.PROCLIB")
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
    ///     .list_dataset_members("SYS1.PROCLIB")
    ///     .attributes_base()
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_dataset_members(
        &self,
        dataset_name: &str,
    ) -> DatasetMemberListBuilder<DatasetMemberList<MemberName>> {
        DatasetMemberListBuilder::new(self.base_url.clone(), self.client.clone(), dataset_name)
    }

    /// # Examples
    ///
    /// Listing datasets:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let list_datasets = zosmf
    ///     .list_datasets("IBMUSER.CONFIG.*")
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
    ///     .list_datasets("**")
    ///     .volume("PEVTS2")
    ///     .attributes_base()
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_datasets(
        &self,
        name_pattern: &str,
    ) -> DatasetListBuilder<DatasetList<DatasetName>> {
        DatasetListBuilder::new(self.base_url.clone(), self.client.clone(), name_pattern)
    }

    pub fn migrate_dataset(&self, name: &str) -> DatasetMigrateBuilder<DatasetMigrate> {
        DatasetMigrateBuilder::new(self.base_url.clone(), self.client.clone(), name)
    }

    /// # Examples
    ///
    /// Reading a PDS member:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let read_member = zosmf
    ///     .read_dataset("SYS1.PARMLIB")
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
    ///     .read_dataset("JIAHJ.REST.SRVMP")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn read_dataset(&self, dataset_name: &str) -> DatasetReadBuilder<DatasetRead<Box<str>>> {
        DatasetReadBuilder::new(self.base_url.clone(), self.client.clone(), dataset_name)
    }

    pub fn recall_dataset(&self, name: &str) -> DatasetRecallBuilder<DatasetRecall> {
        DatasetRecallBuilder::new(self.base_url.clone(), self.client.clone(), name)
    }

    /// # Examples
    ///
    /// Renaming MY.OLD.DSN to MY.NEW.DSN:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let rename_dataset = zosmf
    ///     .rename_dataset("MY.OLD.DSN", "MY.NEW.DSN")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn rename_dataset(
        &self,
        from_dataset: &str,
        to_dataset: &str,
    ) -> DatasetRenameBuilder<DatasetRename> {
        DatasetRenameBuilder::new(
            self.base_url.clone(),
            self.client.clone(),
            from_dataset,
            to_dataset,
        )
    }

    /// # Examples
    ///
    /// Writing to a PDS member:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// # let string_data = "".to_string();
    /// let write_dataset = zosmf
    ///     .write_dataset("SYS1.PARMLIB")
    ///     .member("SMFPRM00")
    ///     .if_match("B5C6454F783590AA8EC15BD88E29EA63")
    ///     .text(string_data)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn write_dataset(&self, dataset_name: &str) -> DatasetWriteBuilder<DatasetWrite> {
        DatasetWriteBuilder::new(self.base_url.clone(), self.client.clone(), dataset_name)
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum DataType {
    Binary,
    Record,
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
                DataType::Record => "record",
                DataType::Text => "text",
            }
        )
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
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

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum ObtainEnq {
    Exclusive,
    SharedReadWrite,
}

impl From<ObtainEnq> for HeaderValue {
    fn from(val: ObtainEnq) -> HeaderValue {
        match val {
            ObtainEnq::Exclusive => "EXCLU",
            ObtainEnq::SharedReadWrite => "SHRW",
        }
        .try_into()
        .unwrap()
    }
}

pub(crate) fn get_session_ref(response: &reqwest::Response) -> Result<Option<Box<str>>, Error> {
    Ok(response
        .headers()
        .get("X-IBM-Session-Ref")
        .map(|v| v.to_str())
        .transpose()?
        .map(|v| v.into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_data_type() {
        assert_eq!(format!("{}", DataType::Binary), "binary");

        assert_eq!(format!("{}", DataType::Record), "record");

        assert_eq!(format!("{}", DataType::Text), "text");
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
        let header_value: HeaderValue = ObtainEnq::Exclusive.into();
        assert_eq!(header_value, HeaderValue::from_static("EXCLU"));

        let header_value: HeaderValue = ObtainEnq::SharedReadWrite.into();
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
