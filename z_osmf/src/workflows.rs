pub mod cancel;
pub mod create;
pub mod definition;
pub mod delete;
pub mod list;
pub mod properties;
pub mod start;

use definition::{WorkflowDefinition, WorkflowDefinitionBuilder};
use delete::WorkflowType;
use serde::{Deserialize, Serialize};

use crate::{ClientCore, Error};

use self::cancel::{WorkflowCancel, WorkflowCancelBuilder};
use self::create::{WorkflowCreate, WorkflowCreateBuilder};
use self::delete::WorkflowDeleteBuilder;
use self::list::{WorkflowList, WorkflowListBuilder};
use self::properties::{WorkflowProperties, WorkflowPropertiesBuilder};
use self::start::WorkflowStartBuilder;

#[derive(Clone, Debug)]
pub struct WorkflowsClient {
    core: ClientCore,
}

impl WorkflowsClient {
    pub(crate) fn new(core: ClientCore) -> Self {
        WorkflowsClient { core }
    }

    pub fn create(
        &self,
        name: &str,
        definition_file: &str,
        system: &str,
        owner: &str,
    ) -> WorkflowCreateBuilder<WorkflowCreate> {
        WorkflowCreateBuilder::new(self.core.clone(), name, definition_file, system, owner)
    }

    pub fn properties(&self, key: &str) -> WorkflowPropertiesBuilder<WorkflowProperties> {
        WorkflowPropertiesBuilder::new(self.core.clone(), key)
    }

    pub fn list(&self) -> WorkflowListBuilder<WorkflowList> {
        WorkflowListBuilder::new(self.core.clone())
    }

    pub fn start(&self, key: &str) -> WorkflowStartBuilder<()> {
        WorkflowStartBuilder::new(self.core.clone(), key)
    }

    pub async fn cancel(&self, key: &str) -> Result<WorkflowCancel, Error> {
        WorkflowCancelBuilder::new(self.core.clone(), key)
            .build()
            .await
    }

    pub async fn delete(&self, key: &str) -> Result<(), Error> {
        WorkflowDeleteBuilder::new(self.core.clone(), WorkflowType::Workflows, key)
            .build()
            .await
    }

    pub fn definition(&self, path: &str) -> WorkflowDefinitionBuilder<WorkflowDefinition> {
        WorkflowDefinitionBuilder::new(self.core.clone(), path)
    }

    pub fn archive(&self) {}

    pub fn list_archived(&self) {}

    pub fn properties_archived(&self) {}

    pub async fn delete_archived(&self, key: &str) -> Result<(), Error> {
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
    InProgress,
    Complete,
    AutomationInProgress,
    Canceled,
}

#[derive(Clone, Debug)]
enum ReturnData {
    Steps,
    StepsVariables,
    Variables,
}
