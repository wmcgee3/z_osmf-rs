pub mod create;

use std::sync::Arc;

use crate::ClientCore;

use self::create::{Create, CreateBuilder};

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
    ) -> CreateBuilder<Create> {
        CreateBuilder::new(self.core.clone(), name, definition_file, system, owner)
    }
}
