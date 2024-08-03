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
use crate::{ClientCore, Error, Result};

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
    core: ClientCore,
}

/// # Jobs
impl JobsClient {
    pub(crate) fn new(core: ClientCore) -> Self {
        JobsClient { core }
    }

    /// # Examples
    ///
    /// Cancel job TESTJOB2 with ID JOB0084:
    /// ```
    /// # use z_osmf::jobs::JobIdentifier;
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let identifier = JobIdentifier::NameId("TESTJOB2".to_string(), "JOB00084".to_string());
    ///
    /// let job_feedback = zosmf
    ///     .jobs()
    ///     .cancel(identifier)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn cancel<I>(&self, identifier: I) -> JobFeedbackBuilder<JobFeedback>
    where
        I: Into<JobIdentifier>,
    {
        JobFeedbackBuilder::new(self.core.clone(), identifier, "cancel")
    }

    /// # Examples
    ///
    /// Cancel and purge the output of job TESTJOBW with ID JOB0085:
    /// ```
    /// # use z_osmf::jobs::JobIdentifier;
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let identifier = JobIdentifier::NameId("TESTJOBW".to_string(), "JOB00085".to_string());
    ///
    /// let job_feedback = zosmf
    ///     .jobs()
    ///     .cancel_and_purge(identifier)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn cancel_and_purge<I>(&self, identifier: I) -> JobPurgeBuilder<JobFeedback>
    where
        I: Into<JobIdentifier>,
    {
        JobPurgeBuilder::new(self.core.clone(), identifier)
    }

    /// # Examples
    ///
    /// Change the message class of job TESTJOBW with ID JOB0023:
    /// ```
    /// # use z_osmf::jobs::JobIdentifier;
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let identifier = JobIdentifier::NameId("TESTJOBW".to_string(), "JOB00023".to_string());
    ///
    /// let job_feedback = zosmf
    ///     .jobs()
    ///     .change_class(identifier, 'A')
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn change_class<I, C>(&self, identifier: I, class: C) -> JobChangeClassBuilder<JobFeedback>
    where
        I: Into<JobIdentifier>,
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
    /// let identifier = JobIdentifier::NameId("TESTJOBW".to_string(), "JOB00023".to_string());
    ///
    /// let job_feedback = zosmf
    ///     .jobs()
    ///     .hold(identifier)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn hold<I>(&self, identifier: I) -> JobFeedbackBuilder<JobFeedback>
    where
        I: Into<JobIdentifier>,
    {
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
    /// let identifier = JobIdentifier::NameId("TESTJOB1".to_string(), "JOB00023".to_string());
    ///
    /// let job_files = zosmf
    ///     .jobs()
    ///     .list_files(identifier)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_files<I>(&self, identifier: I) -> JobFileListBuilder<JobFileList>
    where
        I: Into<JobIdentifier>,
    {
        JobFileListBuilder::new(self.core.clone(), identifier)
    }

    /// # Examples
    ///
    /// Read file 1 for job TESTJOBJ with ID JOB00023:
    /// ```
    /// # use z_osmf::jobs::files::read::JobFileId;
    /// # use z_osmf::jobs::JobIdentifier;
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let identifier = JobIdentifier::NameId("TESTJOBJ".to_string(), "JOB00023".to_string());
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
    /// let identifier = JobIdentifier::NameId("TESTJOBJ".to_string(), "JOB00023".to_string());
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
    /// let identifier = JobIdentifier::NameId("TESTJOBJ".to_string(), "JOB00060".to_string());
    ///
    /// let job_file = zosmf
    ///     .jobs()
    ///     .read_file(identifier, JobFileId::Jcl)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn read_file<I, F>(
        &self,
        identifier: I,
        file_id: F,
    ) -> JobFileReadBuilder<JobFileRead<Arc<str>>>
    where
        I: Into<JobIdentifier>,
        F: Into<JobFileId>,
    {
        JobFileReadBuilder::new(self.core.clone(), identifier, file_id)
    }

    /// # Examples
    ///
    /// Release job TESTJOBW with ID JOB0023:
    /// ```
    /// # use z_osmf::jobs::JobIdentifier;
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let identifier = JobIdentifier::NameId("TESTJOBW".to_string(), "JOB00023".to_string());
    ///
    /// let job_feedback = zosmf
    ///     .jobs()
    ///     .release(identifier)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn release<I>(&self, identifier: I) -> JobFeedbackBuilder<JobFeedback>
    where
        I: Into<JobIdentifier>,
    {
        JobFeedbackBuilder::new(self.core.clone(), identifier, "release")
    }

    /// # Examples
    ///
    /// Obtain the status of the job BLSJPRMI, job ID STC00052:
    /// ```
    /// # use z_osmf::jobs::JobIdentifier;
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let identifier = JobIdentifier::NameId("BLSJPRMI".to_string(), "STC00052".to_string());
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
    pub fn status<I>(&self, identifier: I) -> JobStatusBuilder<JobAttributes>
    where
        I: Into<JobIdentifier>,
    {
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
    pub fn submit<S>(&self, source: S) -> JobSubmitBuilder<JobAttributes>
    where
        S: Into<JobSource>,
    {
        JobSubmitBuilder::new(self.core.clone(), source)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct JobAttributes {
    #[serde(rename = "jobid")]
    id: Arc<str>,
    #[serde(rename = "jobname")]
    name: Arc<str>,
    subsystem: Option<Arc<str>>,
    owner: Arc<str>,
    #[getter(copy)]
    status: Option<JobStatus>,
    #[getter(copy)]
    #[serde(rename = "type")]
    job_type: Option<JobType>,
    class: Arc<str>,
    #[serde(rename = "retcode")]
    return_code: Option<Arc<str>>,
    url: Arc<str>,
    files_url: Arc<str>,
    job_correlator: Option<Arc<str>>,
    #[getter(copy)]
    phase: i32,
    phase_name: Arc<str>,
    reason_not_running: Option<Arc<str>>,
}

impl JobAttributes {
    pub fn identifier(&self) -> JobIdentifier {
        self.into()
    }
}

impl TryFromResponse for JobAttributes {
    async fn try_from_response(value: reqwest::Response) -> Result<Self> {
        Ok(value.json().await?)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct JobAttributesExec {
    #[serde(flatten)]
    job_data: JobAttributes,
    #[serde(default)]
    exec_system: Option<Arc<str>>,
    #[serde(default)]
    exec_member: Option<Arc<str>>,
    #[serde(default)]
    exec_submitted: Option<Arc<str>>,
    #[serde(default)]
    exec_ended: Option<Arc<str>>,
}

impl std::ops::Deref for JobAttributesExec {
    type Target = JobAttributes;

    fn deref(&self) -> &Self::Target {
        &self.job_data
    }
}

impl TryFromResponse for JobAttributesExec {
    async fn try_from_response(value: reqwest::Response) -> Result<Self> {
        Ok(value.json().await?)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct JobAttributesExecStep {
    #[serde(flatten)]
    job_exec_data: JobAttributesExec,
    step_data: Arc<[JobStepData]>,
}

impl std::ops::Deref for JobAttributesExecStep {
    type Target = JobAttributes;

    fn deref(&self) -> &Self::Target {
        &self.job_exec_data
    }
}

impl TryFromResponse for JobAttributesExecStep {
    async fn try_from_response(value: reqwest::Response) -> Result<Self> {
        Ok(value.json().await?)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct JobAttributesStep {
    #[serde(flatten)]
    job_data: JobAttributes,
    step_data: Arc<[JobStepData]>,
}

impl std::ops::Deref for JobAttributesStep {
    type Target = JobAttributes;

    fn deref(&self) -> &Self::Target {
        &self.job_data
    }
}

impl TryFromResponse for JobAttributesStep {
    async fn try_from_response(value: reqwest::Response) -> Result<Self> {
        Ok(value.json().await?)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum JobIdentifier {
    Correlator(String),
    NameId(String, String),
}

impl std::str::FromStr for JobIdentifier {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.split('/').collect::<Vec<_>>()[..] {
            [name, id] => Ok(JobIdentifier::NameId(name.to_string(), id.to_string())),
            [correlator] => Ok(JobIdentifier::Correlator(correlator.to_string())),
            _ => Err(Error::InvalidValue(format!(
                "invalid job identifier: {}",
                s
            ))),
        }
    }
}

impl std::fmt::Display for JobIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JobIdentifier::Correlator(correlator) => write!(f, "{}", correlator),
            JobIdentifier::NameId(name, id) => write!(f, "{}/{}", name, id),
        }
    }
}

impl From<&JobAttributes> for JobIdentifier {
    fn from(value: &JobAttributes) -> Self {
        JobIdentifier::NameId(value.name().to_string(), value.id().to_string())
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
    smf_id: Option<Arc<str>>,
    #[getter(copy)]
    step_number: i32,
    #[serde(default)]
    selected_time: Option<Arc<str>>,
    #[serde(default)]
    owner: Option<Arc<str>>,
    program_name: Arc<str>,
    step_name: Arc<str>,
    #[serde(default)]
    path_name: Option<Arc<str>>,
    #[getter(copy)]
    #[serde(default)]
    substep_number: Option<i32>,
    #[serde(default)]
    end_time: Option<Arc<str>>,
    proc_step_name: Arc<str>,
    #[serde(default, rename = "completion")]
    completion_code: Option<Arc<str>>,
    #[serde(default)]
    abend_reason_code: Option<Arc<str>>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum JobType {
    Job,
    Stc,
    Tsu,
}

fn get_subsystem(value: &Option<Arc<str>>) -> String {
    value
        .as_ref()
        .map(|v| format!("/-{}", v))
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_job_identifier() {
        assert_eq!(
            format!("{}", JobIdentifier::Correlator("ABCD1234".to_string())),
            "ABCD1234"
        );
    }
}
