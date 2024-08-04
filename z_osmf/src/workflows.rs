pub mod archive;
pub mod archived_workflows;
pub mod cancel;
pub mod create;
pub mod definition;
pub mod delete;
pub mod list;
pub mod properties;
pub mod start;

use archive::{WorkflowArchive, WorkflowArchiveBuilder};
use archived_workflows::{ArchivedWorkflowList, ArchivedWorkflowListBuilder};
use definition::{WorkflowDefinition, WorkflowDefinitionBuilder};
use serde::{Deserialize, Serialize};

use crate::{ClientCore, Result};

use self::cancel::{WorkflowCancel, WorkflowCancelBuilder};
use self::create::{WorkflowCreate, WorkflowCreateBuilder};
use self::delete::WorkflowDeleteBuilder;
use self::list::{WorkflowList, WorkflowListBuilder};
use self::properties::{WorkflowProperties, WorkflowPropertiesBuilder};
use self::start::WorkflowStartBuilder;

/// # Workflows
#[derive(Clone, Debug)]
pub struct WorkflowsClient {
    core: ClientCore,
}

impl WorkflowsClient {
    pub(crate) fn new(core: ClientCore) -> Self {
        WorkflowsClient { core }
    }

    /// # Examples
    ///
    /// Create a z/OSMF Workflow:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// # use z_osmf::workflows::WorkflowAccess;
    /// let workflow_create = zosmf
    ///     .workflows()
    ///     .create(
    ///         "AutomationExample",
    ///         "/usr/lpp/zosmf/samples/workflow_sample_automation.xml",
    ///         "SY1",
    ///         "zosmfad",
    ///     )
    ///     .assign_to_owner(true)
    ///     .access_type(WorkflowAccess::Restricted)
    ///     .delete_completed_jobs(true)
    ///     .auto_delete_on_completion(true)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create<N, F, S, O>(
        &self,
        name: N,
        definition_file: F,
        system: S,
        owner: O,
    ) -> WorkflowCreateBuilder<WorkflowCreate>
    where
        N: std::fmt::Display,
        F: std::fmt::Display,
        S: std::fmt::Display,
        O: std::fmt::Display,
    {
        WorkflowCreateBuilder::new(self.core.clone(), name, definition_file, system, owner)
    }

    /// # Examples
    ///
    /// Get the properties of a z/OSMF Workflow:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let workflow_properties = zosmf
    ///     .workflows()
    ///     .properties("26f1fd84-058b-443c-8e06-5ec318ecdb86")
    ///     .steps()
    ///     .variables()
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn properties<K>(&self, key: K) -> WorkflowPropertiesBuilder<WorkflowProperties>
    where
        K: std::fmt::Display,
    {
        WorkflowPropertiesBuilder::new(self.core.clone(), WorkflowType::Workflows, key)
    }

    /// # Examples
    ///
    /// List z/OSMF Workflows on a system or sysplex:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let workflow_list = zosmf
    ///     .workflows()
    ///     .list()
    ///     .name("AutomationExample.*")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(&self) -> WorkflowListBuilder<WorkflowList> {
        WorkflowListBuilder::new(self.core.clone())
    }

    /// # Examples
    ///
    /// Start a z/OSMF Workflow:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// zosmf.workflows()
    ///     .start("d043b5f1-adab-48e7-b7c3-d41cd95fa4b0")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn start<K>(&self, key: K) -> WorkflowStartBuilder<()>
    where
        K: std::fmt::Display,
    {
        WorkflowStartBuilder::new(self.core.clone(), key)
    }

    /// # Examples
    ///
    /// Cancel execution of a z/OSMF Workflow:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let workflow_cancel = zosmf.workflows()
    ///     .cancel("d043b5f1-adab-48e7-b7c3-d41cd95fa4b0")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn cancel<K>(&self, key: K) -> Result<WorkflowCancel>
    where
        K: std::fmt::Display,
    {
        WorkflowCancelBuilder::new(self.core.clone(), key)
            .build()
            .await
    }

    /// # Examples
    ///
    /// Delete a z/OSMF Workflow:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// zosmf.workflows()
    ///     .delete("d043b5f1-adab-48e7-b7c3-d41cd95fa4b0")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete<K>(&self, key: K) -> Result<()>
    where
        K: std::fmt::Display,
    {
        WorkflowDeleteBuilder::new(self.core.clone(), WorkflowType::Workflows, key)
            .build()
            .await
    }

    /// # Examples
    ///
    /// Retrieve a z/OSMF Workflow definition:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let workflow_definition = zosmf
    ///     .workflows()
    ///     .definition("/usr/lpp/zosmf/samples/workflow_sample_program_execution.xml")
    ///     .steps()
    ///     .variables()
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn definition<P>(&self, path: P) -> WorkflowDefinitionBuilder<WorkflowDefinition>
    where
        P: std::fmt::Display,
    {
        WorkflowDefinitionBuilder::new(self.core.clone(), path)
    }

    /// # Examples
    ///
    /// Archive a z/OSMF Workflow:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let workflow_archive = zosmf
    ///     .workflows()
    ///     .archive("2535b19e-a8c3-4a52-9d77-e30bb920f912")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn archive<K>(&self, key: K) -> Result<WorkflowArchive>
    where
        K: std::fmt::Display,
    {
        WorkflowArchiveBuilder::new(self.core.clone(), key)
            .build()
            .await
    }

    /// # Examples
    ///
    /// List archived z/OSMF Workflows:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// # use z_osmf::workflows::archived_workflows::WorkflowOrderBy;
    /// let archived_workflow_list = zosmf
    ///     .workflows()
    ///     .list_archived()
    ///     .order_by(WorkflowOrderBy::Desc)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_archived(&self) -> ArchivedWorkflowListBuilder<ArchivedWorkflowList> {
        ArchivedWorkflowListBuilder::new(self.core.clone())
    }

    /// # Examples
    ///
    /// Get the properties of an archived z/OSMF Workflow:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// # use z_osmf::workflows::archived_workflows::WorkflowOrderBy;
    /// let archived_workflow_properties = zosmf
    ///     .workflows()
    ///     .properties_archived("2535b19e-a8c3-4a52-9d77-e30bb920f912")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn properties_archived<K>(&self, key: K) -> WorkflowPropertiesBuilder<WorkflowProperties>
    where
        K: std::fmt::Display,
    {
        WorkflowPropertiesBuilder::new(self.core.clone(), WorkflowType::ArchivedWorkflows, key)
    }

    /// # Examples
    ///
    /// Delete an archived z/OSMF Workflow:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// zosmf.workflows()
    ///     .delete_archived("7c4bac42-16a3-4af5-a5b9-263e60b280a4")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_archived<K>(&self, key: K) -> Result<()>
    where
        K: std::fmt::Display,
    {
        WorkflowDeleteBuilder::new(self.core.clone(), WorkflowType::ArchivedWorkflows, key)
            .build()
            .await
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum WorkflowAccess {
    Private,
    Public,
    Restricted,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum WorkflowStatus {
    AutomationInProgress,
    Canceled,
    Complete,
    InProgress,
}

#[derive(Clone, Debug)]
enum ReturnData {
    Steps,
    StepsVariables,
    Variables,
}

#[derive(Clone, Debug)]
enum WorkflowType {
    ArchivedWorkflows,
    Workflows,
}

impl std::fmt::Display for WorkflowType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                WorkflowType::ArchivedWorkflows => "archivedworkflows",
                WorkflowType::Workflows => "workflows",
            }
        )
    }
}
