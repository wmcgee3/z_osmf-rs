pub mod create;
pub mod properties;

use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::ClientCore;

use self::create::{WorkflowCreate, WorkflowCreateBuilder};

#[derive(Clone, Debug)]
pub struct WorkflowsClient {
    core: Arc<ClientCore>,
}

impl WorkflowsClient {
    pub(super) fn new(core: Arc<ClientCore>) -> Self {
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

    pub fn properties(&self) {}
}

#[derive(
    Clone, Copy, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub enum WorkflowAccess {
    Private,
    #[default]
    Public,
    Restricted,
}
