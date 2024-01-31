pub use crate::utils::RecordRange;

pub mod list;
pub mod list_files;
pub mod purge;
pub mod read_file;
pub mod status;
pub mod submit;

pub use self::list::*;
pub use self::list_files::*;
pub use self::purge::*;
pub use self::read_file::*;
pub use self::status::*;
pub use self::submit::*;
pub use self::utils::*;

mod utils;

use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct JobsClient {
    base_url: Arc<str>,
    client: reqwest::Client,
}

impl JobsClient {
    pub(crate) fn new(base_url: Arc<str>, client: reqwest::Client) -> Self {
        JobsClient { base_url, client }
    }

    /// # Examples
    ///
    /// Obtain the status of the job BLSJPRMI, job ID STC00052:
    /// ```
    /// # use z_osmf::jobs::JobIdentifier;
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let identifier = JobIdentifier::NameId("BLSJPRMI".into(), "STC00052".into());
    ///
    /// let job_status = zosmf
    ///     .jobs()
    ///     .status(identifier)
    ///     .exec_data()
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn status(&self, identifier: JobIdentifier) -> JobStatusBuilder<JobData> {
        JobStatusBuilder::new(self.base_url.clone(), self.client.clone(), identifier)
    }

    /// # Examples
    ///
    /// List jobs with exec-data by owner and prefix:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let job_list = zosmf
    ///     .jobs()
    ///     .list()
    ///     .owner("IBMUSER")
    ///     .prefix("TESTJOB*")
    ///     .exec_data()
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(&self) -> JobsListBuilder<JobsList<JobData>> {
        JobsListBuilder::new(self.base_url.clone(), self.client.clone())
    }

    /// # Examples
    ///
    /// List the spool files for job TESTJOB1 with ID JOB00023:
    /// ```
    /// # use z_osmf::jobs::JobIdentifier;
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let identifier = JobIdentifier::NameId("TESTJOB1".into(), "JOB00023".into());
    ///
    /// let job_files = zosmf
    ///     .jobs()
    ///     .list_files(identifier)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_files(&self, identifier: JobIdentifier) -> JobsFileListBuilder<JobsFileList> {
        JobsFileListBuilder::new(self.base_url.clone(), self.client.clone(), identifier)
    }

    /// # Examples
    ///
    /// Read file 1 for job TESTJOBJ with ID JOB00023:
    /// ```
    /// # use z_osmf::jobs::{JobIdentifier, JobFileID};
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let identifier = JobIdentifier::NameId("TESTJOBJ".into(), "JOB00023".into());
    /// let file_id = JobFileID::ID(1);
    ///
    /// let job_file = zosmf
    ///     .jobs()
    ///     .read_file(identifier, file_id)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Read a range of records (the first 250) of file 8 for job TESTJOBJ with ID JOB00023:
    /// ```
    /// # use std::str::FromStr;
    /// # use z_osmf::jobs::{JobIdentifier, JobFileID, RecordRange};
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let identifier = JobIdentifier::NameId("TESTJOBJ".into(), "JOB00023".into());
    /// let file_id = JobFileID::ID(8);
    ///
    /// let job_file = zosmf
    ///     .jobs()
    ///     .read_file(identifier, file_id)
    ///     .record_range(RecordRange::from_str("0-249")?)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Read the JCL for job TESTJOBJ with ID JOB00060:
    /// ```
    /// # use z_osmf::jobs::{JobIdentifier, JobFileID};
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let identifier = JobIdentifier::NameId("TESTJOBJ".into(), "JOB00023".into());
    /// let file_id = JobFileID::JCL;
    ///
    /// let job_file = zosmf
    ///     .jobs()
    ///     .read_file(identifier, file_id)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn read_file(
        &self,
        identifier: JobIdentifier,
        id: JobFileID,
    ) -> JobFileReadBuilder<JobFileRead<Box<str>>> {
        JobFileReadBuilder::new(self.base_url.clone(), self.client.clone(), identifier, id)
    }

    /// # Examples
    ///
    /// Submit a job from text:
    /// ```
    /// # use z_osmf::jobs::{JclData, JclSource};
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let jcl = r#"//TESTJOBX JOB (),MSGCLASS=H
    /// // EXEC PGM=IEFBR14
    /// "#;
    ///
    /// let job_data = zosmf
    ///     .jobs()
    ///     .submit(JclSource::Data(JclData::Text(jcl.into())))
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn submit(&self, jcl_source: JclSource) -> JobSubmitBuilder<JobData> {
        JobSubmitBuilder::new(self.base_url.clone(), self.client.clone(), jcl_source)
    }

    /// # Examples
    ///
    /// Hold job TESTJOBW with ID JOB0023:
    /// ```
    /// # use z_osmf::jobs::JobIdentifier;
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let identifier = JobIdentifier::NameId("TESTJOBW".into(), "JOB00023".into());
    ///
    /// let job_feedback = zosmf
    ///     .jobs()
    ///     .hold(identifier)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn hold(&self, identifier: JobIdentifier) -> JobFeedbackBuilder<JobFeedback, RequestJson> {
        JobFeedbackBuilder::new(
            self.base_url.clone(),
            self.client.clone(),
            identifier,
            RequestJson::new("hold"),
        )
    }

    /// # Examples
    ///
    /// Release job TESTJOBW with ID JOB0023:
    /// ```
    /// # use z_osmf::jobs::JobIdentifier;
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let identifier = JobIdentifier::NameId("TESTJOBW".into(), "JOB00023".into());
    ///
    /// let job_feedback = zosmf
    ///     .jobs()
    ///     .release(identifier)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn release(
        &self,
        identifier: JobIdentifier,
    ) -> JobFeedbackBuilder<JobFeedback, RequestJson> {
        JobFeedbackBuilder::new(
            self.base_url.clone(),
            self.client.clone(),
            identifier,
            RequestJson::new("release"),
        )
    }

    /// # Examples
    ///
    /// Change the message class of job TESTJOBW with ID JOB0023:
    /// ```
    /// # use z_osmf::jobs::JobIdentifier;
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let identifier = JobIdentifier::NameId("TESTJOBW".into(), "JOB00023".into());
    ///
    /// let job_feedback = zosmf
    ///     .jobs()
    ///     .change_class(identifier, 'A')
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn change_class<C>(
        &self,
        identifier: JobIdentifier,
        class: C,
    ) -> JobFeedbackBuilder<JobFeedback, ClassJson>
    where
        C: Into<char>,
    {
        JobFeedbackBuilder::new(
            self.base_url.clone(),
            self.client.clone(),
            identifier,
            ClassJson::new(class),
        )
    }

    /// # Examples
    ///
    /// Cancel job TESTJOB2 with ID JOB0084:
    /// ```
    /// # use z_osmf::jobs::JobIdentifier;
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let identifier = JobIdentifier::NameId("TESTJOB2".into(), "JOB00084".into());
    ///
    /// let job_feedback = zosmf
    ///     .jobs()
    ///     .cancel(identifier)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn cancel(
        &self,
        identifier: JobIdentifier,
    ) -> JobFeedbackBuilder<JobFeedback, RequestJson> {
        JobFeedbackBuilder::new(
            self.base_url.clone(),
            self.client.clone(),
            identifier,
            RequestJson::new("cancel"),
        )
    }

    /// # Examples
    ///
    /// Cancel and purge the output of job TESTJOBW with ID JOB0085:
    /// ```
    /// # use z_osmf::jobs::JobIdentifier;
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let identifier = JobIdentifier::NameId("TESTJOBW".into(), "JOB00085".into());
    ///
    /// let job_feedback = zosmf
    ///     .jobs()
    ///     .cancel_and_purge(identifier)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn cancel_and_purge(&self, identifier: JobIdentifier) -> PurgeBuilder<JobFeedback> {
        PurgeBuilder::new(self.base_url.clone(), self.client.clone(), identifier)
    }
}
