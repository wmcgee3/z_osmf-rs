pub mod create;
pub mod list;
pub mod properties;

use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::ClientCore;

use self::create::{WorkflowCreate, WorkflowCreateBuilder};
use self::list::{WorkflowList, WorkflowListBuilder};
use self::properties::{WorkflowProperties, WorkflowPropertiesBuilder};

#[derive(Clone, Debug)]
pub struct WorkflowsClient {
    core: Arc<ClientCore>,
}

impl WorkflowsClient {
    pub(crate) fn new(core: Arc<ClientCore>) -> Self {
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

    pub fn list(&self) -> WorkflowListBuilder<WorkflowList> {
        WorkflowListBuilder::new(self.core.clone())
    }

    pub fn properties(&self, key: &str) -> WorkflowPropertiesBuilder<WorkflowProperties> {
        WorkflowPropertiesBuilder::new(self.core.clone(), key)
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
