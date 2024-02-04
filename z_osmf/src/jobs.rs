pub mod feedback;
pub mod list;
pub mod list_files;
pub mod purge;
pub mod read_file;
pub mod status;
pub mod submit;

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use z_osmf_macros::Getters;

use crate::convert::TryFromResponse;
use crate::error::Error;

use self::feedback::{ClassJson, JobFeedback, JobFeedbackBuilder, RequestJson};
use self::list::{ListJobs, ListJobsBuilder};
use self::list_files::{ListJobFiles, ListJobFilesBuilder};
use self::purge::PurgeJobBuilder;
use self::read_file::{JobFileID, ReadJobFile, ReadJobFileBuilder};
use self::status::JobStatusBuilder;
use self::submit::{JclSource, SubmitJobBuilder};

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
    pub fn cancel_and_purge(&self, identifier: JobIdentifier) -> PurgeJobBuilder<JobFeedback> {
        PurgeJobBuilder::new(self.base_url.clone(), self.client.clone(), identifier)
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
    pub fn list(&self) -> ListJobsBuilder<ListJobs<JobData>> {
        ListJobsBuilder::new(self.base_url.clone(), self.client.clone())
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
    pub fn list_files(&self, identifier: JobIdentifier) -> ListJobFilesBuilder<ListJobFiles> {
        ListJobFilesBuilder::new(self.base_url.clone(), self.client.clone(), identifier)
    }

    /// # Examples
    ///
    /// Read file 1 for job TESTJOBJ with ID JOB00023:
    /// ```
    /// # use z_osmf::jobs::read_file::JobFileID;
    /// # use z_osmf::jobs::JobIdentifier;
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
    /// # use z_osmf::jobs::read_file::{JobFileID, RecordRange};
    /// # use z_osmf::jobs::JobIdentifier;
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
    /// # use z_osmf::jobs::read_file::JobFileID;
    /// # use z_osmf::jobs::JobIdentifier;
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let identifier = JobIdentifier::NameId("TESTJOBJ".into(), "JOB00060".into());
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
    ) -> ReadJobFileBuilder<ReadJobFile<Box<str>>> {
        ReadJobFileBuilder::new(self.base_url.clone(), self.client.clone(), identifier, id)
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
    /// Submit a job from text:
    /// ```
    /// # use z_osmf::jobs::submit::{JclData, JclSource, RecordFormat};
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let jcl = r#"//TESTJOBX JOB (),MSGCLASS=H
    /// // EXEC PGM=IEFBR14
    /// "#;
    ///
    /// let job_data = zosmf
    ///     .jobs()
    ///     .submit(JclSource::Jcl(JclData::Text(jcl.into())))
    ///     .message_class('A')
    ///     .record_format(RecordFormat::Fixed)
    ///     .record_length(80)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn submit(&self, jcl_source: JclSource) -> SubmitJobBuilder<JobData> {
        SubmitJobBuilder::new(self.base_url.clone(), self.client.clone(), jcl_source)
    }
}

pub struct AsynchronousResponse;

impl TryFromResponse for AsynchronousResponse {
    async fn try_from_response(_: reqwest::Response) -> Result<Self, Error> {
        Ok(AsynchronousResponse {})
    }
}

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct JobData {
    #[serde(rename = "jobid")]
    id: Box<str>,
    #[serde(rename = "jobname")]
    name: Box<str>,
    subsystem: Option<Box<str>>,
    owner: Box<str>,
    status: Option<Status>,
    job_type: Option<JobType>,
    class: Box<str>,
    #[serde(rename = "retcode")]
    return_code: Option<Box<str>>,
    url: Box<str>,
    files_url: Box<str>,
    job_correlator: Option<Box<str>>,
    phase: i32,
    phase_name: Box<str>,
    reason_not_running: Option<Box<str>>,
}

impl JobData {
    pub fn identifier(&self) -> JobIdentifier {
        JobIdentifier::NameId(self.name.clone(), self.id.clone())
    }
}

impl TryFromResponse for JobData {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, Error> {
        Ok(value.json().await?)
    }
}

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct JobExecData {
    #[serde(rename = "jobid")]
    id: Box<str>,
    #[serde(rename = "jobname")]
    name: Box<str>,
    subsystem: Option<Box<str>>,
    owner: Box<str>,
    status: Option<Status>,
    job_type: Option<JobType>,
    class: Box<str>,
    #[serde(rename = "retcode")]
    return_code: Option<Box<str>>,
    url: Box<str>,
    files_url: Box<str>,
    job_correlator: Option<Box<str>>,
    phase: i32,
    phase_name: Box<str>,
    exec_system: Box<str>,
    #[serde(default)]
    exec_member: Option<Box<str>>,
    #[serde(default)]
    exec_submitted: Option<Box<str>>,
    #[serde(default)]
    exec_ended: Option<Box<str>>,
    reason_not_running: Option<Box<str>>,
}

impl JobExecData {
    pub fn identifier(&self) -> JobIdentifier {
        JobIdentifier::NameId(self.name.clone(), self.id.clone())
    }
}

impl TryFromResponse for JobExecData {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, Error> {
        Ok(value.json().await?)
    }
}

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct JobExecStepData {
    #[serde(rename = "jobid")]
    id: Box<str>,
    #[serde(rename = "jobname")]
    name: Box<str>,
    subsystem: Option<Box<str>>,
    owner: Box<str>,
    status: Option<Status>,
    job_type: Option<JobType>,
    class: Box<str>,
    #[serde(rename = "retcode")]
    return_code: Option<Box<str>>,
    url: Box<str>,
    files_url: Box<str>,
    job_correlator: Option<Box<str>>,
    phase: i32,
    phase_name: Box<str>,
    step_data: Box<[StepData]>,
    exec_system: Box<str>,
    #[serde(default)]
    exec_member: Option<Box<str>>,
    #[serde(default)]
    exec_submitted: Option<Box<str>>,
    #[serde(default)]
    exec_ended: Option<Box<str>>,
    reason_not_running: Option<Box<str>>,
}

impl JobExecStepData {
    pub fn identifier(&self) -> JobIdentifier {
        JobIdentifier::NameId(self.name.clone(), self.id.clone())
    }
}

impl TryFromResponse for JobExecStepData {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, Error> {
        Ok(value.json().await?)
    }
}

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct JobStepData {
    #[serde(rename = "jobid")]
    id: Box<str>,
    #[serde(rename = "jobname")]
    name: Box<str>,
    subsystem: Option<Box<str>>,
    owner: Box<str>,
    status: Option<Status>,
    job_type: Option<JobType>,
    class: Box<str>,
    #[serde(rename = "retcode")]
    return_code: Option<Box<str>>,
    url: Box<str>,
    files_url: Box<str>,
    job_correlator: Option<Box<str>>,
    phase: i32,
    phase_name: Box<str>,
    step_data: Box<[StepData]>,
    reason_not_running: Option<Box<str>>,
}

impl JobStepData {
    pub fn identifier(&self) -> JobIdentifier {
        JobIdentifier::NameId(self.name.clone(), self.id.clone())
    }
}

impl TryFromResponse for JobStepData {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, Error> {
        Ok(value.json().await?)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum JobIdentifier {
    Correlator(Box<str>),
    NameId(Box<str>, Box<str>),
}

impl std::fmt::Display for JobIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let items = match self {
            JobIdentifier::Correlator(correlator) => vec![correlator.as_ref()],
            JobIdentifier::NameId(name, id) => vec![name.as_ref(), id.as_ref()],
        };

        write!(f, "{}", items.join("/"))
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum JobType {
    #[serde(rename = "JOB")]
    Job,
    #[serde(rename = "STC")]
    StartedTask,
    #[serde(rename = "TSU")]
    TsoUser,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Status {
    Active,
    Input,
    Output,
}

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct StepData {
    active: bool,
    #[serde(rename = "smfid")]
    smf_id: Box<str>,
    step_number: i32,
    #[serde(default)]
    selected_time: Option<Box<str>>,
    owner: Box<str>,
    program_name: Box<str>,
    step_name: Box<str>,
    #[serde(default)]
    path_name: Option<Box<str>>,
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
            format!("{}", JobIdentifier::Correlator("ABCD1234".into())),
            "ABCD1234"
        );
    }
}
