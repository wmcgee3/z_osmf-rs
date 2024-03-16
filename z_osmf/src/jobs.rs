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

use self::class::ClassBuilder;
use self::feedback::{Feedback, FeedbackBuilder};
use self::files::{FileId, JobFiles, JobFilesBuilder, Read, ReadBuilder};
use self::list::{Jobs, JobsBuilder};
use self::purge::PurgeBuilder;
use self::status::StatusBuilder;
use self::submit::{Jcl, SubmitBuilder};

#[derive(Clone, Debug)]
pub struct JobsClient {
    core: Arc<ClientCore>,
}

/// # Jobs
impl JobsClient {
    pub(crate) fn new(core: &Arc<ClientCore>) -> Self {
        JobsClient { core: core.clone() }
    }

    /// # Examples
    ///
    /// Cancel job TESTJOB2 with ID JOB0084:
    /// ```
    /// # use z_osmf::jobs::Identifier;
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let identifier = Identifier::NameId("TESTJOB2".to_string(), "JOB00084".to_string());
    ///
    /// let job_feedback = zosmf
    ///     .jobs()
    ///     .cancel(identifier)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn cancel(&self, identifier: Identifier) -> FeedbackBuilder<Feedback> {
        FeedbackBuilder::new(self.core.clone(), identifier, "cancel")
    }

    /// # Examples
    ///
    /// Cancel and purge the output of job TESTJOBW with ID JOB0085:
    /// ```
    /// # use z_osmf::jobs::Identifier;
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let identifier = Identifier::NameId("TESTJOBW".to_string(), "JOB00085".to_string());
    ///
    /// let job_feedback = zosmf
    ///     .jobs()
    ///     .cancel_and_purge(identifier)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn cancel_and_purge(&self, identifier: Identifier) -> PurgeBuilder<Feedback> {
        PurgeBuilder::new(self.core.clone(), identifier)
    }

    /// # Examples
    ///
    /// Change the message class of job TESTJOBW with ID JOB0023:
    /// ```
    /// # use z_osmf::jobs::Identifier;
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let identifier = Identifier::NameId("TESTJOBW".to_string(), "JOB00023".to_string());
    ///
    /// let job_feedback = zosmf
    ///     .jobs()
    ///     .change_class(identifier, 'A')
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn change_class<C>(&self, identifier: Identifier, class: C) -> ClassBuilder<Feedback>
    where
        C: Into<char>,
    {
        ClassBuilder::new(self.core.clone(), identifier, class)
    }

    /// # Examples
    ///
    /// Hold job TESTJOBW with ID JOB0023:
    /// ```
    /// # use z_osmf::jobs::Identifier;
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let identifier = Identifier::NameId("TESTJOBW".to_string(), "JOB00023".to_string());
    ///
    /// let job_feedback = zosmf
    ///     .jobs()
    ///     .hold(identifier)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn hold(&self, identifier: Identifier) -> FeedbackBuilder<Feedback> {
        FeedbackBuilder::new(self.core.clone(), identifier, "hold")
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
    pub fn list(&self) -> JobsBuilder<Jobs<Job>> {
        JobsBuilder::new(self.core.clone())
    }

    /// # Examples
    ///
    /// List the spool files for job TESTJOB1 with ID JOB00023:
    /// ```
    /// # use z_osmf::jobs::Identifier;
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let identifier = Identifier::NameId("TESTJOB1".to_string(), "JOB00023".to_string());
    ///
    /// let job_files = zosmf
    ///     .jobs()
    ///     .list_files(identifier)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_files(&self, identifier: Identifier) -> JobFilesBuilder<JobFiles> {
        JobFilesBuilder::new(self.core.clone(), identifier)
    }

    /// # Examples
    ///
    /// Read file 1 for job TESTJOBJ with ID JOB00023:
    /// ```
    /// # use z_osmf::jobs::files::Id;
    /// # use z_osmf::jobs::Identifier;
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let identifier = Identifier::NameId("TESTJOBJ".to_string(), "JOB00023".to_string());
    ///
    /// let job_file = zosmf
    ///     .jobs()
    ///     .read_file(identifier, Id::Id(1))
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Read a range of records (the first 250) of file 8 for job TESTJOBJ with ID JOB00023:
    /// ```
    /// # use std::str::FromStr;
    /// # use z_osmf::jobs::files::{Id, RecordRange};
    /// # use z_osmf::jobs::Identifier;
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let identifier = Identifier::NameId("TESTJOBJ".to_string(), "JOB00023".to_string());
    ///
    /// let job_file = zosmf
    ///     .jobs()
    ///     .read_file(identifier, Id::Id(8))
    ///     .record_range(RecordRange::from_str("0-249")?)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Read the JCL for job TESTJOBJ with ID JOB00060:
    /// ```
    /// # use z_osmf::jobs::files::Id;
    /// # use z_osmf::jobs::Identifier;
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let identifier = Identifier::NameId("TESTJOBJ".to_string(), "JOB00060".to_string());
    ///
    /// let job_file = zosmf
    ///     .jobs()
    ///     .read_file(identifier, Id::Jcl)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn read_file(&self, identifier: Identifier, id: FileId) -> ReadBuilder<Read<Box<str>>> {
        ReadBuilder::new(self.core.clone(), identifier, id)
    }

    /// # Examples
    ///
    /// Release job TESTJOBW with ID JOB0023:
    /// ```
    /// # use z_osmf::jobs::Identifier;
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let identifier = Identifier::NameId("TESTJOBW".to_string(), "JOB00023".to_string());
    ///
    /// let job_feedback = zosmf
    ///     .jobs()
    ///     .release(identifier)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn release(&self, identifier: Identifier) -> FeedbackBuilder<Feedback> {
        FeedbackBuilder::new(self.core.clone(), identifier, "release")
    }

    /// # Examples
    ///
    /// Obtain the status of the job BLSJPRMI, job ID STC00052:
    /// ```
    /// # use z_osmf::jobs::Identifier;
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let identifier = Identifier::NameId("BLSJPRMI".to_string(), "STC00052".to_string());
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
    pub fn status(&self, identifier: Identifier) -> StatusBuilder<Job> {
        StatusBuilder::new(self.core.clone(), identifier)
    }

    /// # Examples
    ///
    /// Submit a job from text:
    /// ```
    /// # use z_osmf::jobs::submit::{Jcl, RecordFormat};
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let jcl = r#"//TESTJOBX JOB (),MSGCLASS=H
    /// // EXEC PGM=IEFBR14
    /// "#;
    ///
    /// let job_data = zosmf
    ///     .jobs()
    ///     .submit(Jcl::Text(jcl.into()))
    ///     .message_class('A')
    ///     .record_format(RecordFormat::Fixed)
    ///     .record_length(80)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn submit(&self, jcl_source: Jcl) -> SubmitBuilder<Job> {
        SubmitBuilder::new(self.core.clone(), jcl_source)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct AsynchronousResponse;

impl TryFromResponse for AsynchronousResponse {
    async fn try_from_response(_: reqwest::Response) -> Result<Self, Error> {
        Ok(AsynchronousResponse {})
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Job {
    #[serde(rename = "jobid")]
    id: Box<str>,
    #[serde(rename = "jobname")]
    name: Box<str>,
    subsystem: Option<Box<str>>,
    owner: Box<str>,
    #[getter(copy)]
    status: Option<Status>,
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

impl Job {
    pub fn identifier(&self) -> Identifier {
        Identifier::NameId(self.name.to_string(), self.id.to_string())
    }
}

impl TryFromResponse for Job {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, Error> {
        Ok(value.json().await?)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct JobExec {
    #[serde(flatten)]
    job_data: Job,
    #[serde(default)]
    exec_system: Option<Box<str>>,
    #[serde(default)]
    exec_member: Option<Box<str>>,
    #[serde(default)]
    exec_submitted: Option<Box<str>>,
    #[serde(default)]
    exec_ended: Option<Box<str>>,
}

impl std::ops::Deref for JobExec {
    type Target = Job;

    fn deref(&self) -> &Self::Target {
        &self.job_data
    }
}

impl TryFromResponse for JobExec {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, Error> {
        Ok(value.json().await?)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct JobExecStep {
    #[serde(flatten)]
    job_exec_data: JobExec,
    step_data: Box<[Step]>,
}

impl std::ops::Deref for JobExecStep {
    type Target = Job;

    fn deref(&self) -> &Self::Target {
        &self.job_exec_data
    }
}

impl TryFromResponse for JobExecStep {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, Error> {
        Ok(value.json().await?)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct JobStep {
    #[serde(flatten)]
    job_data: Job,
    step_data: Box<[Step]>,
}

impl std::ops::Deref for JobStep {
    type Target = Job;

    fn deref(&self) -> &Self::Target {
        &self.job_data
    }
}

impl TryFromResponse for JobStep {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, Error> {
        Ok(value.json().await?)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize)]
pub enum Identifier {
    Correlator(String),
    NameId(String, String),
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let items = match self {
            Identifier::Correlator(correlator) => vec![correlator.as_ref()],
            Identifier::NameId(name, id) => vec![name.as_ref(), id.as_ref()],
        };

        write!(f, "{}", items.join("/"))
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Status {
    Active,
    Input,
    Output,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize)]
pub enum JobType {
    #[serde(rename = "JOB")]
    Job,
    #[serde(rename = "STC")]
    StartedTask,
    #[serde(rename = "TSU")]
    TsoUser,
}

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Step {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_job_identifier() {
        assert_eq!(
            format!("{}", Identifier::Correlator("ABCD1234".into())),
            "ABCD1234"
        );
    }
}
