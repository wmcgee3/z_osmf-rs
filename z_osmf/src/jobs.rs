pub mod list;
pub mod status;

pub use self::list::*;
pub use self::status::*;

use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::convert::TryFromResponse;
use crate::error::Error;

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
    /// List jobs with exec-data by owner and prefix:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let list_jobs = zosmf
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
    /// Obtain the status of the job BLSJPRMI, job ID STC00052:
    /// ```
    /// # use z_osmf::jobs::Identifier;
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let identifier = Identifier::NameId("BLSJPRMI".into(), "STC00052".into());
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
    pub fn status(&self, identifier: Identifier) -> JobStatusBuilder<JobData> {
        JobStatusBuilder::new(self.base_url.clone(), self.client.clone(), identifier)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct JobData {
    #[serde(rename = "jobid")]
    pub id: Box<str>,
    #[serde(rename = "jobname")]
    pub name: Box<str>,
    pub subsystem: Option<Box<str>>,
    pub owner: Box<str>,
    pub status: Option<Status>,
    pub job_type: Option<JobType>,
    pub class: Box<str>,
    #[serde(rename = "retcode")]
    pub return_code: Option<Box<str>>,
    pub url: Box<str>,
    pub files_url: Box<str>,
    pub job_correlator: Option<Box<str>>,
    pub phase: i32,
    pub phase_name: Box<str>,
    pub reason_not_running: Option<Box<str>>,
}

impl JobData {
    pub fn identifier(&self) -> Identifier {
        Identifier::NameId(self.name.clone(), self.id.clone())
    }
}

impl TryFromResponse for JobData {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, Error> {
        Ok(value.json().await?)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct JobExecData {
    #[serde(rename = "jobid")]
    pub id: Box<str>,
    #[serde(rename = "jobname")]
    pub name: Box<str>,
    pub subsystem: Option<Box<str>>,
    pub owner: Box<str>,
    pub status: Option<Status>,
    pub job_type: Option<JobType>,
    pub class: Box<str>,
    #[serde(rename = "retcode")]
    pub return_code: Option<Box<str>>,
    pub url: Box<str>,
    pub files_url: Box<str>,
    pub job_correlator: Option<Box<str>>,
    pub phase: i32,
    pub phase_name: Box<str>,
    pub exec_system: Box<str>,
    pub exec_member: Box<str>,
    pub exec_submitted: Box<str>,
    pub exec_ended: Box<str>,
    pub reason_not_running: Option<Box<str>>,
}

impl JobExecData {
    pub fn identifier(&self) -> Identifier {
        Identifier::NameId(self.name.clone(), self.id.clone())
    }
}

impl TryFromResponse for JobExecData {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, Error> {
        Ok(value.json().await?)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct JobExecStepData {
    #[serde(rename = "jobid")]
    pub id: Box<str>,
    #[serde(rename = "jobname")]
    pub name: Box<str>,
    pub subsystem: Option<Box<str>>,
    pub owner: Box<str>,
    pub status: Option<Status>,
    pub job_type: Option<JobType>,
    pub class: Box<str>,
    #[serde(rename = "retcode")]
    pub return_code: Option<Box<str>>,
    pub url: Box<str>,
    pub files_url: Box<str>,
    pub job_correlator: Option<Box<str>>,
    pub phase: i32,
    pub phase_name: Box<str>,
    pub step_data: Vec<StepData>,
    pub exec_system: Box<str>,
    pub exec_member: Box<str>,
    pub exec_submitted: Box<str>,
    pub exec_ended: Box<str>,
    pub reason_not_running: Option<Box<str>>,
}

impl JobExecStepData {
    pub fn identifier(&self) -> Identifier {
        Identifier::NameId(self.name.clone(), self.id.clone())
    }
}

impl TryFromResponse for JobExecStepData {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, Error> {
        Ok(value.json().await?)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct JobStepData {
    #[serde(rename = "jobid")]
    pub id: Box<str>,
    #[serde(rename = "jobname")]
    pub name: Box<str>,
    pub subsystem: Option<Box<str>>,
    pub owner: Box<str>,
    pub status: Option<Status>,
    pub job_type: Option<JobType>,
    pub class: Box<str>,
    #[serde(rename = "retcode")]
    pub return_code: Option<Box<str>>,
    pub url: Box<str>,
    pub files_url: Box<str>,
    pub job_correlator: Option<Box<str>>,
    pub phase: i32,
    pub phase_name: Box<str>,
    pub step_data: Vec<StepData>,
    pub reason_not_running: Option<Box<str>>,
}

impl JobStepData {
    pub fn identifier(&self) -> Identifier {
        Identifier::NameId(self.name.clone(), self.id.clone())
    }
}

impl TryFromResponse for JobStepData {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, Error> {
        Ok(value.json().await?)
    }
}

pub enum Identifier {
    NameId(Box<str>, Box<str>),
    Correlator(Box<str>),
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        type JI = Identifier;
        let items = match self {
            JI::Correlator(correlator) => vec![correlator.as_ref()],
            JI::NameId(name, id) => vec![name.as_ref(), id.as_ref()],
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

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct StepData {
    pub active: bool,
    #[serde(rename = "smfid")]
    pub smf_id: Box<str>,
    pub step_number: i32,
    #[serde(default)]
    pub selected_time: Option<Box<str>>,
    pub owner: Box<str>,
    pub program_name: Box<str>,
    pub step_name: Box<str>,
    #[serde(default)]
    pub path_name: Option<Box<str>>,
    #[serde(default)]
    pub substep_number: Option<i32>,
    #[serde(default)]
    pub end_time: Option<Box<str>>,
    pub proc_step_name: Box<str>,
    #[serde(default, rename = "completion")]
    pub completion_code: Option<Box<str>>,
    #[serde(default)]
    pub abend_reason_code: Option<Box<str>>,
}
