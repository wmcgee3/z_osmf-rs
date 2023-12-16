use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
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

impl From<JobData> for Identifier {
    fn from(value: JobData) -> Self {
        Identifier::NameId(value.name, value.id)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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
    exec_member: Box<str>,
    exec_submitted: Box<str>,
    exec_ended: Box<str>,
    reason_not_running: Option<Box<str>>,
}

impl From<JobExecData> for Identifier {
    fn from(value: JobExecData) -> Self {
        Identifier::NameId(value.name, value.id)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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
    step_data: Vec<StepData>,
    exec_system: Box<str>,
    exec_member: Box<str>,
    exec_submitted: Box<str>,
    exec_ended: Box<str>,
    reason_not_running: Option<Box<str>>,
}

impl From<JobExecStepData> for Identifier {
    fn from(value: JobExecStepData) -> Self {
        Identifier::NameId(value.name, value.id)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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
    step_data: Vec<StepData>,
    reason_not_running: Option<Box<str>>,
}

impl From<JobStepData> for Identifier {
    fn from(value: JobStepData) -> Self {
        Identifier::NameId(value.name, value.id)
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
