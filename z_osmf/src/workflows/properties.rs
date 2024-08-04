use std::marker::PhantomData;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use z_osmf_macros::{Endpoint, Getters};

use crate::convert::TryFromResponse;
use crate::jobs::{JobStatus, JobType};
use crate::{ClientCore, Result};

use super::{ReturnData, WorkflowAccess, WorkflowStatus, WorkflowType};

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowAutomationStatus {
    start_user: Arc<str>,
    #[getter(copy)]
    started_time: u64,
    #[getter(copy)]
    stopped_time: Option<u64>,
    current_step_name: Option<Arc<str>>,
    current_step_number: Option<Arc<str>>,
    current_step_title: Option<Arc<str>>,
    #[serde(rename = "messageID")]
    message_id: Option<Arc<str>>,
    message_text: Option<Arc<str>>,
}

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowProperties {
    #[serde(rename = "workflowName")]
    name: Arc<str>,
    #[serde(rename = "workflowKey")]
    key: Arc<str>,
    #[serde(rename = "workflowDescription")]
    description: Arc<str>,
    #[serde(rename = "workflowID")]
    id: Arc<str>,
    #[serde(rename = "workflowVersion")]
    version: Arc<str>,
    #[serde(rename = "workflowDefinitionFileMD5Value")]
    definition_file_hash: Arc<str>,
    vendor: Arc<str>,
    owner: Arc<str>,
    #[serde(rename = "workflowArchiveSAFID")]
    archive_saf_id: Option<Arc<str>>,
    system: Arc<str>,
    jobs_output_directory: Option<Arc<str>>,
    category: Arc<str>,
    #[serde(rename = "productID")]
    product_id: Option<Arc<str>>,
    product_name: Option<Arc<str>>,
    product_version: Option<Arc<str>>,
    #[getter(copy)]
    percent_complete: i32,
    is_callable: Option<Arc<str>>,
    #[getter(copy)]
    contains_parallel_steps: bool,
    #[getter(copy)]
    scope: WorkflowScope,
    #[getter(copy)]
    #[serde(rename = "statusName")]
    status: WorkflowStatus,
    #[getter(copy)]
    delete_completed_jobs: bool,
    automation_status: Option<WorkflowAutomationStatus>,
    #[getter(copy)]
    auto_delete_on_completion: Option<bool>,
    #[getter(copy)]
    access: WorkflowAccess,
    account_info: Option<Arc<str>>,
    job_statement: Option<Arc<str>>,
    template_id: Option<Arc<str>>,
    action_id: Option<Arc<str>>,
    registry_id: Option<Arc<str>>,
    parent_registry_id: Option<Arc<str>>,
    domain_id: Option<Arc<str>>,
    tenant_id: Option<Arc<str>>,
    #[serde(rename = "software-service-instance-name")]
    software_service_instance_name: Option<Arc<str>>,
    template_name: Option<Arc<str>>,
    global_variable_group: Option<Arc<str>>,
    #[getter(copy)]
    is_instance_variable_without_prefix: bool,
    software_type: Option<Arc<str>>,
}

impl TryFromResponse for WorkflowProperties {
    async fn try_from_response(value: reqwest::Response) -> Result<Self> {
        Ok(value.json().await?)
    }
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = get, path = "/zosmf/workflow/rest/1.0/{workflow_type}/{key}")]
pub struct WorkflowPropertiesBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(path, skip_setter)]
    workflow_type: WorkflowType,
    #[endpoint(path)]
    key: Arc<str>,
    #[endpoint(skip_setter, builder_fn = build_return_data)]
    return_data: Option<ReturnData>,

    target_type: PhantomData<T>,
}

impl WorkflowPropertiesBuilder<WorkflowProperties> {
    pub fn steps(self) -> WorkflowPropertiesBuilder<WorkflowPropertiesSteps> {
        WorkflowPropertiesBuilder {
            core: self.core,
            workflow_type: self.workflow_type,
            key: self.key,
            return_data: Some(ReturnData::Steps),
            target_type: PhantomData,
        }
    }

    pub fn variables(self) -> WorkflowPropertiesBuilder<WorkflowPropertiesVariables> {
        WorkflowPropertiesBuilder {
            core: self.core,
            workflow_type: self.workflow_type,
            key: self.key,
            return_data: Some(ReturnData::Variables),
            target_type: PhantomData,
        }
    }
}

impl WorkflowPropertiesBuilder<WorkflowPropertiesSteps> {
    pub fn variables(self) -> WorkflowPropertiesBuilder<WorkflowPropertiesStepsVariables> {
        WorkflowPropertiesBuilder {
            core: self.core,
            workflow_type: self.workflow_type,
            key: self.key,
            return_data: Some(ReturnData::StepsVariables),
            target_type: PhantomData,
        }
    }
}

impl WorkflowPropertiesBuilder<WorkflowPropertiesVariables> {
    pub fn steps(self) -> WorkflowPropertiesBuilder<WorkflowPropertiesStepsVariables> {
        WorkflowPropertiesBuilder {
            core: self.core,
            workflow_type: self.workflow_type,
            key: self.key,
            return_data: Some(ReturnData::StepsVariables),
            target_type: PhantomData,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct WorkflowPropertiesSteps {
    #[getter(skip)]
    #[serde(flatten)]
    core: WorkflowProperties,
    steps: Arc<[WorkflowStep]>,
}

impl std::ops::Deref for WorkflowPropertiesSteps {
    type Target = WorkflowProperties;

    fn deref(&self) -> &Self::Target {
        &self.core
    }
}

impl TryFromResponse for WorkflowPropertiesSteps {
    async fn try_from_response(value: reqwest::Response) -> Result<Self> {
        Ok(value.json().await?)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct WorkflowPropertiesStepsVariables {
    #[getter(skip)]
    #[serde(flatten)]
    core: WorkflowProperties,
    steps: Arc<[WorkflowStep]>,
    variables: Arc<[WorkflowVariableInfo]>,
}

impl std::ops::Deref for WorkflowPropertiesStepsVariables {
    type Target = WorkflowProperties;

    fn deref(&self) -> &Self::Target {
        &self.core
    }
}

impl TryFromResponse for WorkflowPropertiesStepsVariables {
    async fn try_from_response(value: reqwest::Response) -> Result<Self> {
        Ok(value.json().await?)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct WorkflowPropertiesVariables {
    #[getter(skip)]
    #[serde(flatten)]
    core: WorkflowProperties,
    variables: Arc<[WorkflowVariableInfo]>,
}

impl std::ops::Deref for WorkflowPropertiesVariables {
    type Target = WorkflowProperties;

    fn deref(&self) -> &Self::Target {
        &self.core
    }
}

impl TryFromResponse for WorkflowPropertiesVariables {
    async fn try_from_response(value: reqwest::Response) -> Result<Self> {
        Ok(value.json().await?)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum WorkflowScope {
    System,
    Sysplex,
    None,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(untagged)]
pub enum WorkflowStep {
    Calling(WorkflowStepCalling),
    Rest(WorkflowStepRest),
    Template(WorkflowStepTemplate),
}

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowStepCalling {
    #[getter(skip)]
    #[serde(flatten)]
    core: WorkflowStepNonRest,
    called_instance_key: Option<Arc<str>>,
    #[getter(copy)]
    called_instance_scope: Option<WorkflowScope>,
    called_instance_url: Option<Arc<str>>,
    called_workflow_id: Option<Arc<str>>,
    called_workflow_version: Option<Arc<str>>,
    #[serde(rename = "calledWorkflowMD5")]
    called_workflow_hash: Option<Arc<str>>,
    called_workflow_description: Arc<str>,
    called_workflow_definition_file: Option<Arc<str>>,
}

impl std::ops::Deref for WorkflowStepCalling {
    type Target = WorkflowStepNonRest;

    fn deref(&self) -> &Self::Target {
        &self.core
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowStepCore {
    name: Arc<str>,
    #[getter(copy)]
    auto_enable: bool,
    description: Arc<str>,
    #[getter(copy)]
    #[serde(default)]
    is_rest_step: bool,
    #[getter(copy)]
    optional: bool,
    prereq_step: Option<Arc<[Arc<str>]>>,
    run_as_user: Option<Arc<str>>,
    #[getter(copy)]
    run_as_user_dynamic: Option<bool>,
    #[getter(copy)]
    state: WorkflowStepStatus,
    step_number: Arc<str>,
    steps: Option<Arc<[WorkflowStep]>>,
    title: Arc<str>,
    #[getter(copy)]
    user_defined: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct WorkflowStepJobInfo {
    #[serde(rename = "jobstatus")]
    status: WorkflowStepJobInfoStatus,
    #[serde(rename = "jobfiles")]
    files: Option<Arc<[WorkflowStepJobInfoFile]>>,
}

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct WorkflowStepJobInfoFile {
    #[serde(rename = "ddname")]
    dd_name: Arc<str>,
    #[serde(rename = "stepname")]
    step_name: Arc<str>,
    id: i32,
    record_count: i32,
    class: Arc<str>,
    byte_count: i32,
    #[serde(rename = "procstep")]
    proc_step: Option<Arc<str>>,
}

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct WorkflowStepJobInfoStatus {
    #[serde(rename = "retcode")]
    return_code: Option<Arc<str>>,
    #[serde(rename = "jobname")]
    name: Arc<str>,
    #[getter(copy)]
    status: Option<JobStatus>,
    owner: Arc<str>,
    subsystem: Option<Arc<str>>,
    class: Arc<str>,
    #[getter(copy)]
    #[serde(rename = "type")]
    job_type: JobType,
    #[serde(rename = "jobid")]
    id: Arc<str>,
}

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowStepNonRest {
    #[getter(skip)]
    #[serde(flatten)]
    core: WorkflowStepCore,
    assignees: Option<Arc<str>>,
    #[getter(copy)]
    has_called_workflow: bool,
    #[getter(copy)]
    is_condition_step: Option<bool>,
    owner: Option<Arc<str>>,
    skills: Option<Arc<str>>,
    weight: Arc<str>,
}

impl std::ops::Deref for WorkflowStepNonRest {
    type Target = WorkflowStepCore;

    fn deref(&self) -> &Self::Target {
        &self.core
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowStepRest {
    #[getter(skip)]
    #[serde(flatten)]
    core: WorkflowStepCore,
    actual_status_code: Arc<str>,
    expected_status_code: Arc<str>,
    hostname: Arc<str>,
    http_method: Arc<str>,
    port: Arc<str>,
    query_parameters: Arc<str>,
    #[getter(copy)]
    query_parameters_sub: bool,
    request_body: Arc<str>,
    #[getter(copy)]
    request_body_sub: bool,
    scheme_name: Arc<str>,
    #[getter(copy)]
    scheme_name_sub: bool,
    uri_path: Arc<str>,
    #[getter(copy)]
    uri_path_sub: bool,
}

impl std::ops::Deref for WorkflowStepRest {
    type Target = WorkflowStepCore;

    fn deref(&self) -> &Self::Target {
        &self.core
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum WorkflowStepStatus {
    Unassigned,
    Assigned,
    #[serde(rename = "Not Ready")]
    NotReady,
    Ready,
    #[serde(rename = "In Progress")]
    InProgress,
    Submitted,
    Complete,
    Skipped,
    #[serde(rename = "Complete (Override)")]
    CompleteOverride,
    Failed,
    Conflicts,
    #[serde(rename = "Condition Not Satisfied")]
    ConditionNotSatisfied,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE")]
pub enum WorkflowStepSubmitAs {
    Jcl,
    TsoRexx,
    #[serde(rename = "shell-JCL")]
    ShellJcl,
    TsoRexxJcl,
    TsoUnixRexx,
    #[serde(rename = "TSO-UNIX-shell")]
    TsoUnixShell,
}

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowStepTemplate {
    #[getter(skip)]
    #[serde(flatten)]
    core: WorkflowStepNonRest,
    failed_pattern: Option<Arc<[Arc<str>]>>,
    instructions: Option<Arc<str>>,
    #[getter(copy)]
    instructions_sub: Option<bool>,
    job_info: Option<WorkflowStepJobInfo>,
    #[getter(copy)]
    max_lrecl: Option<i32>,
    output: Option<Arc<str>>,
    #[getter(copy)]
    output_sub: Option<bool>,
    output_variables_prefix: Option<Arc<str>>,
    proc_name: Option<Arc<str>>,
    #[getter(copy)]
    region_size: Option<i32>,
    return_code: Option<Arc<str>>,
    save_as_dataset: Option<Arc<str>>,
    #[getter(copy)]
    save_as_dataset_sub: Option<bool>,
    save_as_unix_file: Option<Arc<str>>,
    #[getter(copy)]
    save_as_unix_file_sub: Option<bool>,
    script_parameters: Option<Arc<str>>,
    #[getter(copy)]
    submit_as: Option<WorkflowStepSubmitAs>,
    success_pattern: Option<Arc<str>>,
    template: Option<Arc<str>>,
    #[getter(copy)]
    template_sub: Option<bool>,
    #[getter(copy)]
    timeout: Option<i32>,
    #[serde(rename = "variable-references")]
    variable_references: Option<Arc<[WorkflowStepVariableReference]>>,
}

impl std::ops::Deref for WorkflowStepTemplate {
    type Target = WorkflowStepNonRest;

    fn deref(&self) -> &Self::Target {
        &self.core
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct WorkflowStepVariableReference {
    name: Arc<str>,
    #[getter(copy)]
    scope: WorkflowVariableScope,
}

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct WorkflowVariableInfo {
    name: Arc<str>,
    #[getter(copy)]
    scope: WorkflowVariableScope,
    #[getter(copy)]
    #[serde(rename = "type")]
    variable_type: WorkflowVariableType,
    value: Option<Arc<str>>,
    visibility: Arc<str>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum WorkflowVariableScope {
    Instance,
    Global,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum WorkflowVariableType {
    Boolean,
    String,
    Number,
    Date,
    Time,
    Array,
}

fn build_return_data<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &WorkflowPropertiesBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    match &builder.return_data {
        Some(ReturnData::Steps) => request_builder.query(&[("returnData", "steps")]),
        Some(ReturnData::StepsVariables) => {
            request_builder.query(&[("returnData", "steps,variables")])
        }
        Some(ReturnData::Variables) => request_builder.query(&[("returnData", "variables")]),
        None => request_builder,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_response() -> anyhow::Result<()> {
        let json_text: &str = r###"
        {
            "access": "Public",
            "productID": "ABC123",
            "jobStatement": null,
            "deleteCompletedJobs": false,
            "productName": "Product ABC",
            "globalVariableGroup": null,
            "productVersion": "Version 1",
            "jobsOutputDirectory": null,
            "vendor": "IBM",
            "scope": "none",
            "statusName": "in-progress",
            "workflowID": "programExecutionSample",
            "owner": "zosmfad",
            "accountInfo": null,
            "isInstanceVariableWithoutPrefix": false,
            "variables": [
              {
                "visibility": "private",
                "scope": "instance",
                "name": "procNameVariable",
                "type": "string",
                "value": null
              },
              {
                "visibility": "private",
                "scope": "instance",
                "name": "st_group",
                "type": "string",
                "value": null
              },
              {
                "visibility": "private",
                "scope": "instance",
                "name": "st_user",
                "type": "string",
                "value": null
              }
            ],
            "workflowName": "testProgramExecutionSample",
            "automationStatus": null,
            "autoDeleteOnCompletion": false,
            "percentComplete": 0,
            "workflowDescription": "Sample that demonstrates how to run an executable program from a step.\n\t",
            "steps": [
              {
                "template": "#!/bin/sh\necho \"this is a sample to submit a shell script to run immediately\"\necho                                         \"the first parameter is :\" $1 \t\necho ${instance-st_user}                                          \necho prefix:st_group = SYS123\necho prefix:st_user = USERS\necho                                          \"This symbol is used to indicate success\"\t\necho                                          \"The program ran successfully!\"",
                "instructions": "This step outputs some variables and prints a few words.\n        ",
                "maxLrecl": 1024,
                "failedPattern": ["failed.*"],
                "assignees": "zosmfad",
                "description": "In this step, you submit an inline UNIX shell script for immediate processing \n\t\ton the host system.                            In this example, the step is expected to complete successfully.\n\t\t",
                "outputVariablesPrefix": "prefix:",
                "variable-references": [
                  {
                    "scope": "instance",
                    "name": "st_group"
                  },
                  {
                    "scope": "instance",
                    "name": "st_user"
                  },
                  {
                    "scope": "instance",
                    "name": "procNameVariable"
                  }
                ],
                "saveAsUnixFileSub": true,
                "procName": "${instance-procNameVariable}",
                "title": "A step that runs a UNIX shell script.",
                "jobInfo": null,
                "timeout": 60,
                "regionSize": 50000,
                "skills": "System Programmer",
                "isRestStep": false,
                "output": null,
                "outputSub": false,
                "returnCode": null,
                "outputSysoutDD": false,
                "successPattern": "success.*",
                "state": "Ready",
                "templateSub": true,
                "owner": "zosmfad",
                "autoEnable": false,
                "submitAs": "TSO-UNIX-shell",
                "userDefined": false,
                "weight": "1",
                "optional": false,
                "steps": null,
                "scriptParameters": "para1",
                "saveAsUnixFile": "/u/${instance-st_user}/savedStuff/myScript.sh",
                "instructionsSub": false,
                "saveAsDatasetSub": false,
                "isConditionStep": false,
                "prereqStep": null,
                "hasCalledWorkflow": false,
                "name": "TSO-UNIX-shell_Execution",
                "stepNumber": "1",
                "saveAsDataset": null
              },
              {
                "template": "/*  rexx  */\nparse arg arg1                                   \nSAY \"this is a sample to submit a UNIX REXX script to run immediately\"                                   \nSAY \"the first parameter is :\" arg1\nSAY ${instance-st_user}                                   \nSAY \"prefix:st_group =\" SYS123\nSAY \"prefix:st_user =\" USERS                                   \nSAY \"This symbol is used to indicate failed\"",
                "instructions": "This step outputs some variables and prints a few words.\n        ",
                "maxLrecl": 1024,
                "failedPattern": ["failed.*"],
                "assignees": "zosmfad",
                "description": "In this step, you submit an inline UNIX REXX exec for immediate processing \n\t\ton the host system.                             In this example, the step is expected to fail.\n\t\t",
                "outputVariablesPrefix": "prefix:",
                "variable-references": [
                  {
                    "scope": "instance",
                    "name": "st_group"
                  },
                  {
                    "scope": "instance",
                    "name": "st_user"
                  },
                  {
                    "scope": "instance",
                    "name": "procNameVariable"
                  }
                ],
                "saveAsUnixFileSub": true,
                "procName": "${instance-procNameVariable}",
                "title": "A step that runs a UNIX REXX exec program.",
                "jobInfo": null,
                "timeout": 60,
                "regionSize": 50000,
                "skills": "System Programmer",
                "isRestStep": false,
                "output": null,
                "outputSub": false,
                "returnCode": null,
                "outputSysoutDD": false,
                "successPattern": "success.*",
                "state": "Ready",
                "templateSub": true,
                "owner": "zosmfad",
                "autoEnable": false,
                "submitAs": "TSO-UNIX-REXX",
                "userDefined": false,
                "weight": "1",
                "optional": false,
                "steps": null,
                "scriptParameters": "para1",
                "saveAsUnixFile": "/u/${instance-st_user}/savedStuff/myScript.sh",
                "instructionsSub": false,
                "saveAsDatasetSub": false,
                "isConditionStep": false,
                "prereqStep": null,
                "hasCalledWorkflow": false,
                "name": "TSO-UNIX-REXX_Execution",
                "stepNumber": "2",
                "saveAsDataset": null
              },
              {
                "template": "/*  rexx  */\nparse arg arg1                                   \nSAY \"this is a sample to submit TSO REXX script to run immediately\"                                   \nSAY \"the first parameter is :\" arg1\nSAY ${instance-st_user}                                   \nSAY \"prefix:st_group =\" SYS123\nSAY \"prefix:st_user =\" USERS                                   \nSAY \"This execution will meets timeout.\"",
                "instructions": "This step outputs some variables and prints a few words.\n        ",
                "maxLrecl": 1024,
                "failedPattern": ["failed.*"],
                "assignees": "zosmfad",
                "description": "In this step, you submit an inline REXX exec for immediate processing \n\t\ton the host system. In this example, the processing is ended by a time-out condition.\n\t\t",
                "outputVariablesPrefix": "prefix:",
                "variable-references": [
                  {
                    "scope": "instance",
                    "name": "st_group"
                  },
                  {
                    "scope": "instance",
                    "name": "st_user"
                  },
                  {
                    "scope": "instance",
                    "name": "procNameVariable"
                  }
                ],
                "saveAsUnixFileSub": true,
                "procName": "${instance-procNameVariable}",
                "title": "A step that runs a REXX exec program.",
                "jobInfo": null,
                "timeout": 60,
                "regionSize": 50000,
                "skills": "System Programmer",
                "isRestStep": false,
                "output": null,
                "outputSub": false,
                "returnCode": null,
                "outputSysoutDD": false,
                "successPattern": "success.*",
                "state": "Ready",
                "templateSub": true,
                "owner": "zosmfad",
                "autoEnable": false,
                "submitAs": "TSO-REXX",
                "userDefined": false,
                "weight": "1",
                "optional": false,
                "steps": null,
                "scriptParameters": "para1",
                "saveAsUnixFile": "/u/${instance-st_user}/savedStuff/myScript.sh",
                "instructionsSub": false,
                "saveAsDatasetSub": false,
                "isConditionStep": false,
                "prereqStep": null,
                "hasCalledWorkflow": false,
                "name": "TSO-TSO-REXX_Execution",
                "stepNumber": "3",
                "saveAsDataset": null
              }
            ],
            "containsParallelSteps": false,
            "workflowDefinitionFileMD5Value": "5c5dd66eb3ca3cd1c578ccf323d57cc0",
            "isCallable": null,
            "system": "PLEX1.SY1",
            "workflowKey": "7a2263a7-7c91-40b4-8892-2a4342a222c3",
            "workflowVersion": "1.0",
            "category": "configuration"
        }
"###;

        serde_json::from_str::<WorkflowPropertiesStepsVariables>(json_text)?;

        Ok(())
    }
}
