pub mod create;
pub mod export;
pub mod list;
pub mod symbols;

mod delete;
mod import;

use std::sync::Arc;

use crate::{ClientCore, Error};

use self::create::{CreateBuilder, NewVariable};
use self::delete::DeleteBuilder;
use self::export::ExportBuilder;
use self::import::ImportBuilder;
use self::list::{Variables, VariablesBuilder};
use self::symbols::{Symbols, SymbolsBuilder};

#[derive(Clone, Debug)]
pub struct VariablesClient {
    core: Arc<ClientCore>,
}

impl VariablesClient {
    pub(crate) fn new(core: Arc<ClientCore>) -> Self {
        VariablesClient { core }
    }

    pub async fn create<T, U, V>(
        &self,
        sysplex: T,
        system: U,
        new_variables: V,
    ) -> Result<(), Error>
    where
        T: ToString,
        U: ToString,
        V: Into<Box<[NewVariable]>>,
    {
        CreateBuilder::new(
            self.core.clone(),
            sysplex.to_string(),
            system.to_string(),
            new_variables,
        )
        .build()
        .await
    }

    pub async fn delete<T, U, V>(
        &self,
        sysplex: T,
        system: U,
        variable_names: V,
    ) -> Result<(), Error>
    where
        T: ToString,
        U: ToString,
        V: Into<Box<[String]>>,
    {
        DeleteBuilder::new(
            self.core.clone(),
            sysplex.to_string(),
            system.to_string(),
            variable_names,
        )
        .build()
        .await
    }

    pub fn export<T, U, V>(&self, sysplex: T, system: U, path: V) -> ExportBuilder<()>
    where
        T: ToString,
        U: ToString,
        V: ToString,
    {
        ExportBuilder::new(
            self.core.clone(),
            sysplex.to_string(),
            system.to_string(),
            path.to_string(),
        )
    }

    pub async fn import<T, U, V>(&self, sysplex: T, system: U, path: V) -> Result<(), Error>
    where
        T: ToString,
        U: ToString,
        V: ToString,
    {
        ImportBuilder::new(
            self.core.clone(),
            sysplex.to_string(),
            system.to_string(),
            path.to_string(),
        )
        .build()
        .await
    }

    pub fn list(&self) -> VariablesBuilder<Variables> {
        VariablesBuilder::new(self.core.clone())
    }

    pub fn symbols(&self) -> SymbolsBuilder<Symbols> {
        SymbolsBuilder::new(self.core.clone())
    }
}
