use std::marker::PhantomData;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use z_osmf_macros::{Endpoint, Getters};

use crate::convert::TryFromResponse;
use crate::{ClientCore, Result};

use super::ReturnData;

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowDefinition {
    #[serde(rename = "workflowDefaultName")]
    default_name: Option<Arc<str>>,
    #[serde(rename = "workflowDescription")]
    description: Arc<str>,
    #[serde(rename = "workflowID")]
    id: Arc<str>,
    #[serde(rename = "workflowVersion")]
    version: Arc<str>,
    vendor: Arc<str>,
    #[serde(rename = "workflowDefinitionFileMD5Value")]
    hash: Arc<str>,
    is_callable: Option<Arc<str>>,
    #[getter(copy)]
    contains_parallel_steps: bool,
    scope: Arc<str>,
    jobs_output_directory: Option<Arc<str>>,
    category: Arc<str>,
    #[serde(rename = "productID")]
    product_id: Option<Arc<str>>,
    product_name: Option<Arc<str>>,
    product_version: Option<Arc<str>>,
    global_variable_group: Option<Arc<str>>,
    #[getter(copy)]
    is_instance_variable_without_prefix: bool,
    software_type: Option<Arc<str>>,
}

impl TryFromResponse for WorkflowDefinition {
    async fn try_from_response(value: reqwest::Response) -> Result<Self> {
        Ok(value.json().await?)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct WorkflowDefinitionSteps {
    #[getter(skip)]
    #[serde(flatten)]
    inner: WorkflowDefinition,
    steps: Arc<[StepCore]>,
}

impl std::ops::Deref for WorkflowDefinitionSteps {
    type Target = WorkflowDefinition;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl TryFromResponse for WorkflowDefinitionSteps {
    async fn try_from_response(value: reqwest::Response) -> Result<Self> {
        Ok(value.json().await?)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct WorkflowDefinitionStepsVariables {
    #[getter(skip)]
    #[serde(flatten)]
    inner: WorkflowDefinitionSteps,
    variables: Arc<[VariableDefinition]>,
}

impl std::ops::Deref for WorkflowDefinitionStepsVariables {
    type Target = WorkflowDefinitionSteps;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl TryFromResponse for WorkflowDefinitionStepsVariables {
    async fn try_from_response(value: reqwest::Response) -> Result<Self> {
        Ok(value.json().await?)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct WorkflowDefinitionVariables {
    #[getter(skip)]
    #[serde(flatten)]
    inner: WorkflowDefinition,
    variables: Arc<[VariableDefinition]>,
}

impl std::ops::Deref for WorkflowDefinitionVariables {
    type Target = WorkflowDefinition;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl TryFromResponse for WorkflowDefinitionVariables {
    async fn try_from_response(value: reqwest::Response) -> Result<Self> {
        let text: String = value.text().await?;
        std::fs::write("output.json", &text).unwrap();

        Ok(serde_json::from_str(&text).unwrap())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StepCore {
    name: Arc<str>,
    title: Arc<str>,
    description: Arc<str>,
    prereq_step: Option<Arc<[Arc<str>]>>,
    optional: bool,
    steps: Option<Arc<[StepCore]>>,
}

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct VariableDefinition {
    name: Arc<str>,
    scope: Arc<str>,
    #[serde(rename = "abstract")]
    short_description: Arc<str>,
    category: Arc<str>,
    choice: Option<Arc<[Arc<str>]>>,
    decimal_places: Option<i32>,
    default: Option<Arc<str>>,
    description: Arc<str>,
    expose_to_user: bool,
    max_length: Option<i32>,
    max_value: Option<Arc<str>>,
    min_length: Option<i32>,
    min_value: Option<Arc<str>>,
    prompt_at_create: bool,
    regular_expression: Option<Arc<str>>,
    required_at_create: bool,
    #[serde(rename = "type")]
    variable_type: Arc<str>,
    validation_type: Option<Arc<str>>,
    value_must_be_choice: Option<bool>,
    visibility: Arc<str>,
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = get, path = "/zosmf/workflow/rest/1.0/workflowDefinition?definitionFilePath={path}")]
pub struct WorkflowDefinitionBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(path)]
    path: Arc<str>,
    #[endpoint(query = "workflowDefinitionFileSystem")]
    file_system: Option<Arc<str>>,
    #[endpoint(skip_setter, builder_fn = build_return_data)]
    return_data: Option<ReturnData>,

    target_type: PhantomData<T>,
}

impl WorkflowDefinitionBuilder<WorkflowDefinition> {
    pub fn steps(self) -> WorkflowDefinitionBuilder<WorkflowDefinitionSteps> {
        WorkflowDefinitionBuilder {
            core: self.core,
            path: self.path,
            file_system: self.file_system,
            return_data: Some(ReturnData::Steps),
            target_type: PhantomData,
        }
    }

    pub fn variables(self) -> WorkflowDefinitionBuilder<WorkflowDefinitionVariables> {
        WorkflowDefinitionBuilder {
            core: self.core,
            path: self.path,
            file_system: self.file_system,
            return_data: Some(ReturnData::Variables),
            target_type: PhantomData,
        }
    }
}

impl WorkflowDefinitionBuilder<WorkflowDefinitionSteps> {
    pub fn variables(self) -> WorkflowDefinitionBuilder<WorkflowDefinitionStepsVariables> {
        WorkflowDefinitionBuilder {
            core: self.core,
            path: self.path,
            file_system: self.file_system,
            return_data: Some(ReturnData::StepsVariables),
            target_type: PhantomData,
        }
    }
}

impl WorkflowDefinitionBuilder<WorkflowDefinitionVariables> {
    pub fn steps(self) -> WorkflowDefinitionBuilder<WorkflowDefinitionStepsVariables> {
        WorkflowDefinitionBuilder {
            core: self.core,
            path: self.path,
            file_system: self.file_system,
            return_data: Some(ReturnData::StepsVariables),
            target_type: PhantomData,
        }
    }
}

fn build_return_data<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &WorkflowDefinitionBuilder<T>,
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
