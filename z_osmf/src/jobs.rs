pub mod class;
pub mod feedback;
pub mod files;
pub mod list;
pub mod purge;
pub mod status;
pub mod submit;

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use z_osmf_macros::Getters;

use crate::convert::TryFromResponse;
use crate::error::Error;
use crate::ClientCore;

use self::class::JobChangeClassBuilder;
use self::feedback::{JobFeedback, JobFeedbackBuilder};
use self::files::read::{JobFileId, JobFileRead, JobFileReadBuilder};
use self::files::{JobFileList, JobFileListBuilder};
use self::list::{JobList, JobListBuilder};
use self::purge::JobPurgeBuilder;
use self::status::JobStatusBuilder;
use self::submit::{JobSource, JobSubmitBuilder};

#[derive(Clone, Debug)]
pub struct JobsClient {
    core: Arc<ClientCore>,
}

/// # Jobs
impl JobsClient {
    pub(crate) fn new(core: Arc<ClientCore>) -> Self {
        JobsClient { core }
    }

    /// # Examples
    ///
    /// Cancel job TESTJOB2 with ID JOB0084:
    /// ```
    /// # use z_osmf::jobs::JobIdentifier;
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let identifier = JobIdentifier::NameId("TESTJOB2", "JOB00084");
    ///
    /// let job_feedback = zosmf
    ///     .jobs()
    ///     .cancel(identifier)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn cancel<'a>(&self, identifier: JobIdentifier<'a>) -> JobFeedbackBuilder<'a, JobFeedback> {
        JobFeedbackBuilder::new(self.core.clone(), identifier, "cancel")
    }

    /// # Examples
    ///
    /// Cancel and purge the output of job TESTJOBW with ID JOB0085:
    /// ```
    /// # use z_osmf::jobs::JobIdentifier;
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let identifier = JobIdentifier::NameId("TESTJOBW", "JOB00085");
    ///
    /// let job_feedback = zosmf
    ///     .jobs()
    ///     .cancel_and_purge(identifier)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn cancel_and_purge<'a>(
        &self,
        identifier: JobIdentifier<'a>,
    ) -> JobPurgeBuilder<'a, JobFeedback> {
        JobPurgeBuilder::new(self.core.clone(), identifier)
    }

    /// # Examples
    ///
    /// Change the message class of job TESTJOBW with ID JOB0023:
    /// ```
    /// # use z_osmf::jobs::JobIdentifier;
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let identifier = JobIdentifier::NameId("TESTJOBW", "JOB00023");
    ///
    /// let job_feedback = zosmf
    ///     .jobs()
    ///     .change_class(identifier, 'A')
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn change_class<'a, C>(
        &self,
        identifier: JobIdentifier<'a>,
        class: C,
    ) -> JobChangeClassBuilder<'a, JobFeedback>
    where
        C: Into<char>,
    {
        JobChangeClassBuilder::new(self.core.clone(), identifier, class)
    }

    /// # Examples
    ///
    /// Hold job TESTJOBW with ID JOB0023:
    /// ```
    /// # use z_osmf::jobs::JobIdentifier;
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let identifier = JobIdentifier::NameId("TESTJOBW", "JOB00023");
    ///
    /// let job_feedback = zosmf
    ///     .jobs()
    ///     .hold(identifier)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn hold<'a>(&self, identifier: JobIdentifier<'a>) -> JobFeedbackBuilder<'a, JobFeedback> {
        JobFeedbackBuilder::new(self.core.clone(), identifier, "hold")
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
    pub fn list(&self) -> JobListBuilder<JobList<JobAttributes>> {
        JobListBuilder::new(self.core.clone())
    }

    /// # Examples
    ///
    /// List the spool files for job TESTJOB1 with ID JOB00023:
    /// ```
    /// # use z_osmf::jobs::JobIdentifier;
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let identifier = JobIdentifier::NameId("TESTJOB1", "JOB00023");
    ///
    /// let job_files = zosmf
    ///     .jobs()
    ///     .list_files(identifier)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_files<'a>(
        &self,
        identifier: JobIdentifier<'a>,
    ) -> JobFileListBuilder<'a, JobFileList> {
        JobFileListBuilder::new(self.core.clone(), identifier)
    }

    /// # Examples
    ///
    /// Read file 1 for job TESTJOBJ with ID JOB00023:
    /// ```
    /// # use z_osmf::jobs::files::read::JobFileId;
    /// # use z_osmf::jobs::JobIdentifier;
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let identifier = JobIdentifier::NameId("TESTJOBJ", "JOB00023");
    ///
    /// let job_file = zosmf
    ///     .jobs()
    ///     .read_file(identifier, JobFileId::Id(1))
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Read a range of records (the first 250) of file 8 for job TESTJOBJ with ID JOB00023:
    /// ```
    /// # use std::str::FromStr;
    /// # use z_osmf::jobs::files::read::{JobFileId, RecordRange};
    /// # use z_osmf::jobs::JobIdentifier;
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let identifier = JobIdentifier::NameId("TESTJOBJ", "JOB00023");
    ///
    /// let job_file = zosmf
    ///     .jobs()
    ///     .read_file(identifier, JobFileId::Id(8))
    ///     .record_range(RecordRange::from_str("0-249")?)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Read the JCL for job TESTJOBJ with ID JOB00060:
    /// ```
    /// # use z_osmf::jobs::files::read::JobFileId;
    /// # use z_osmf::jobs::JobIdentifier;
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let identifier = JobIdentifier::NameId("TESTJOBJ", "JOB00060");
    ///
    /// let job_file = zosmf
    ///     .jobs()
    ///     .read_file(identifier, JobFileId::Jcl)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn read_file<'a>(
        &self,
        identifier: JobIdentifier<'a>,
        id: JobFileId,
    ) -> JobFileReadBuilder<'a, JobFileRead<Box<str>>> {
        JobFileReadBuilder::new(self.core.clone(), identifier, id)
    }

    /// # Examples
    ///
    /// Release job TESTJOBW with ID JOB0023:
    /// ```
    /// # use z_osmf::jobs::JobIdentifier;
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let identifier = JobIdentifier::NameId("TESTJOBW", "JOB00023");
    ///
    /// let job_feedback = zosmf
    ///     .jobs()
    ///     .release(identifier)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn release<'a>(
        &self,
        identifier: JobIdentifier<'a>,
    ) -> JobFeedbackBuilder<'a, JobFeedback> {
        JobFeedbackBuilder::new(self.core.clone(), identifier, "release")
    }

    /// # Examples
    ///
    /// Obtain the status of the job BLSJPRMI, job ID STC00052:
    /// ```
    /// # use z_osmf::jobs::JobIdentifier;
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let identifier = JobIdentifier::NameId("BLSJPRMI", "STC00052");
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
    pub fn status<'a>(&self, identifier: JobIdentifier<'a>) -> JobStatusBuilder<'a, JobAttributes> {
        JobStatusBuilder::new(self.core.clone(), identifier)
    }

    /// # Examples
    ///
    /// Submit a job from text:
    /// ```
    /// # use z_osmf::jobs::submit::{JclData, JobRecordFormat, JobSource};
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let jcl = r#"//TESTJOBX JOB (),MSGCLASS=H
    /// // EXEC PGM=IEFBR14
    /// "#;
    ///
    /// let job_data = zosmf
    ///     .jobs()
    ///     .submit(JobSource::Jcl(JclData::Text(jcl.into())))
    ///     .message_class('A')
    ///     .record_format(JobRecordFormat::Fixed)
    ///     .record_length(80)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn submit(&self, jcl_source: JobSource) -> JobSubmitBuilder<JobAttributes> {
        JobSubmitBuilder::new(self.core.clone(), jcl_source)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct JobAttributes {
    #[serde(rename = "jobid")]
    id: Box<str>,
    #[serde(rename = "jobname")]
    name: Box<str>,
    subsystem: Option<Box<str>>,
    owner: Box<str>,
    #[getter(copy)]
    status: Option<JobStatus>,
    #[getter(copy)]
    job_type: Option<JobType>,
    class: Box<str>,
    #[serde(rename = "retcode")]
    return_code: Option<Box<str>>,
    url: Box<str>,
    files_url: Box<str>,
    job_correlator: Option<Box<str>>,
    #[getter(copy)]
    phase: i32,
    phase_name: Box<str>,
    reason_not_running: Option<Box<str>>,
}

impl JobAttributes {
    pub fn identifier(&self) -> JobIdentifier {
        JobIdentifier::NameId(&self.name, &self.id)
    }
}

impl TryFromResponse for JobAttributes {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, Error> {
        Ok(value.json().await?)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct JobAttributesExec {
    #[serde(flatten)]
    job_data: JobAttributes,
    #[serde(default)]
    exec_system: Option<Box<str>>,
    #[serde(default)]
    exec_member: Option<Box<str>>,
    #[serde(default)]
    exec_submitted: Option<Box<str>>,
    #[serde(default)]
    exec_ended: Option<Box<str>>,
}

impl std::ops::Deref for JobAttributesExec {
    type Target = JobAttributes;

    fn deref(&self) -> &Self::Target {
        &self.job_data
    }
}

impl TryFromResponse for JobAttributesExec {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, Error> {
        Ok(value.json().await?)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct JobAttributesExecStep {
    #[serde(flatten)]
    job_exec_data: JobAttributesExec,
    step_data: Box<[JobStepData]>,
}

impl std::ops::Deref for JobAttributesExecStep {
    type Target = JobAttributes;

    fn deref(&self) -> &Self::Target {
        &self.job_exec_data
    }
}

impl TryFromResponse for JobAttributesExecStep {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, Error> {
        Ok(value.json().await?)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct JobAttributesStep {
    #[serde(flatten)]
    job_data: JobAttributes,
    step_data: Box<[JobStepData]>,
}

impl std::ops::Deref for JobAttributesStep {
    type Target = JobAttributes;

    fn deref(&self) -> &Self::Target {
        &self.job_data
    }
}

impl TryFromResponse for JobAttributesStep {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, Error> {
        Ok(value.json().await?)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum JobIdentifier<'a> {
    Correlator(&'a str),
    NameId(&'a str, &'a str),
}

impl<'a> std::fmt::Display for JobIdentifier<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let items = match self {
            JobIdentifier::Correlator(correlator) => vec![*correlator],
            JobIdentifier::NameId(name, id) => vec![*name, *id],
        };

        write!(f, "{}", items.join("/"))
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum JobStatus {
    Active,
    Input,
    Output,
}

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct JobStepData {
    #[getter(copy)]
    active: bool,
    #[serde(default, rename = "smfid")]
    smf_id: Option<Box<str>>,
    #[getter(copy)]
    step_number: i32,
    #[serde(default)]
    selected_time: Option<Box<str>>,
    #[serde(default)]
    owner: Option<Box<str>>,
    program_name: Box<str>,
    step_name: Box<str>,
    #[serde(default)]
    path_name: Option<Box<str>>,
    #[getter(copy)]
    #[serde(default)]
    substep_number: Option<i32>,
    #[serde(default)]
    end_time: Option<Box<str>>,
    proc_step_name: Box<str>,
    #[serde(default, rename = "completion")]
    completion_code: Option<Box<str>>,
    #[serde(default)]
    abend_reason_code: Option<Box<str>>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum JobType {
    Job,
    Stc,
    Tsu,
}

fn get_subsystem(value: &Option<Box<str>>) -> String {
    value
        .as_ref()
        .map(|v| format!("/-{}", v))
        .unwrap_or("".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_job_identifier() {
        assert_eq!(
            format!("{}", JobIdentifier::Correlator("ABCD1234")),
            "ABCD1234"
        );
    }
}
