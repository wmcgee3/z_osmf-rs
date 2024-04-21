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
use crate::restfiles::Etag;
use crate::ClientCore;

use self::copy::DatasetCopyBuilder;
use self::copy_file::DatasetCopyFileBuilder;
use self::create::DatasetCreateBuilder;
use self::delete::DatasetDeleteBuilder;
use self::list::{DatasetAttributesDsName, DatasetList, DatasetsBuilder};
use self::members::{MemberAttributesName, MemberList, MemberListBuilder};
use self::migrate::DatasetMigrateBuilder;
use self::read::{DatasetRead, DatasetReadBuilder};
use self::recall::DatasetRecallBuilder;
use self::rename::DatasetRenameBuilder;
use self::write::DatasetWriteBuilder;

#[derive(Clone, Debug)]
pub struct DatasetsClient {
    core: Arc<ClientCore>,
}

/// # Datasets
impl DatasetsClient {
    pub(super) fn new(core: Arc<ClientCore>) -> Self {
        DatasetsClient { core }
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
    pub fn copy(&self, from_dataset: &str, to_dataset: &str) -> DatasetCopyBuilder<String> {
        DatasetCopyBuilder::new(self.core.clone(), from_dataset, to_dataset)
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
    pub fn copy_file(&self, from_path: &str, to_dataset: &str) -> DatasetCopyFileBuilder<String> {
        DatasetCopyFileBuilder::new(self.core.clone(), from_path, to_dataset)
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
    pub fn create(&self, dataset: &str) -> DatasetCreateBuilder<String> {
        DatasetCreateBuilder::new(self.core.clone(), dataset)
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
    pub fn delete(&self, dataset: &str) -> DatasetDeleteBuilder<String> {
        DatasetDeleteBuilder::new(self.core.clone(), dataset)
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
    pub fn list(&self, level: &str) -> DatasetsBuilder<DatasetList<DatasetAttributesDsName>> {
        DatasetsBuilder::new(self.core.clone(), level)
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
    pub fn members(&self, dataset: &str) -> MemberListBuilder<MemberList<MemberAttributesName>> {
        MemberListBuilder::new(self.core.clone(), dataset)
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
    pub fn migrate(&self, dataset: &str) -> DatasetMigrateBuilder<Etag> {
        DatasetMigrateBuilder::new(self.core.clone(), dataset)
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
    pub fn read(&self, dataset: &str) -> DatasetReadBuilder<DatasetRead<Box<str>>> {
        DatasetReadBuilder::new(self.core.clone(), dataset)
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
    pub fn recall(&self, dataset: &str) -> DatasetRecallBuilder<String> {
        DatasetRecallBuilder::new(self.core.clone(), dataset)
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
    pub fn rename(&self, from_dataset: &str, to_dataset: &str) -> DatasetRenameBuilder<String> {
        DatasetRenameBuilder::new(self.core.clone(), from_dataset, to_dataset)
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
    pub fn write(&self, dataset: &str) -> DatasetWriteBuilder<Etag> {
        DatasetWriteBuilder::new(self.core.clone(), dataset)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
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

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
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

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Enqueue {
    Exclu,
    Shrw,
}

impl From<Enqueue> for HeaderValue {
    fn from(val: Enqueue) -> HeaderValue {
        match val {
            Enqueue::Exclu => "EXCLU",
            Enqueue::Shrw => "SHRW",
        }
        .try_into()
        .unwrap()
    }
}

fn de_optional_y_n<'de, D>(deserializer: D) -> core::result::Result<Option<bool>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Option::<String>::deserialize(deserializer)?
        .map(|s| match s.as_str() {
            "Y" => Ok(true),
            "N" => Ok(false),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Y", "N"])),
        })
        .transpose()
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

fn ser_optional_y_n<S>(v: &Option<bool>, serializer: S) -> core::result::Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match v {
        Some(value) => serializer.serialize_str(if *value { "Y" } else { "N" }),
        None => serializer.serialize_none(),
    }
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
        let header_value: HeaderValue = Enqueue::Exclu.into();
        assert_eq!(header_value, HeaderValue::from_static("EXCLU"));

        let header_value: HeaderValue = Enqueue::Shrw.into();
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

    #[test]
    fn test_de_optional_y_n() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct Test {
            #[serde(default, deserialize_with = "de_optional_y_n")]
            value: Option<bool>,
        }

        assert_eq!(
            serde_json::from_str::<Test>(r#"{"value": "Y"}"#).unwrap(),
            Test { value: Some(true) }
        );

        assert_eq!(
            serde_json::from_str::<Test>(r#"{"value": "N"}"#).unwrap(),
            Test { value: Some(false) }
        );

        assert_eq!(
            serde_json::from_str::<Test>(r#"{"value": null}"#).unwrap(),
            Test { value: None }
        );

        assert_eq!(
            serde_json::from_str::<Test>(r#"{}"#).unwrap(),
            Test { value: None }
        );

        assert!(serde_json::from_str::<Test>(r#"{"value": "NOPE"}"#).is_err());
    }

    #[test]
    fn test_ser_optional_y_n() {
        let mut serializer = serde_json::Serializer::new(Vec::new());
        ser_optional_y_n(&Some(true), &mut serializer).unwrap();
        let serialized = String::from_utf8(serializer.into_inner()).unwrap();
        assert_eq!(serialized, r#""Y""#);

        let mut serializer = serde_json::Serializer::new(Vec::new());
        ser_optional_y_n(&Some(false), &mut serializer).unwrap();
        let serialized = String::from_utf8(serializer.into_inner()).unwrap();
        assert_eq!(serialized, r#""N""#);

        let mut serializer = serde_json::Serializer::new(Vec::new());
        ser_optional_y_n(&None, &mut serializer).unwrap();
        let serialized = String::from_utf8(serializer.into_inner()).unwrap();
        assert_eq!(serialized, r#"null"#);
    }
}
