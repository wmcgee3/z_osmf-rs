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

    /// # Examples
    ///
    /// Create system variables:
    /// ```
    /// # use z_osmf::variables::create::NewVariable;
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let new_variables = [
    ///     NewVariable::new("var1", "value1", "description of the variable"),
    ///     NewVariable::new("var2", "value2", "description of the variable"),
    /// ];
    ///
    /// zosmf.variables()
    ///     .create("TESTPLEX", "TESTNODE", new_variables)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create<T>(
        &self,
        sysplex: &str,
        system: &str,
        new_variables: T,
    ) -> Result<(), Error>
    where
        T: Into<Box<[NewVariable]>>,
    {
        CreateBuilder::new(self.core.clone(), sysplex, system, new_variables)
            .build()
            .await
    }

    /// # Examples
    ///
    /// Delete system variables:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let variable_names = [
    ///     "var1".to_string(),
    ///     "var2".to_string(),
    /// ];
    ///
    /// zosmf.variables()
    ///     .delete("TESTPLEX", "TESTNODE", variable_names)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete<T>(
        &self,
        sysplex: &str,
        system: &str,
        variable_names: T,
    ) -> Result<(), Error>
    where
        T: Into<Box<[String]>>,
    {
        DeleteBuilder::new(self.core.clone(), sysplex, system, variable_names)
            .build()
            .await
    }

    /// # Examples
    ///
    /// Export system variables to a CSV file and overwrite the file if it already exists:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// zosmf.variables()
    ///     .export("TESTPLEX", "TESTNODE", "/u/testuser/backup-variables.csv")
    ///     .overwrite(true)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn export(&self, sysplex: &str, system: &str, path: &str) -> ExportBuilder<()> {
        ExportBuilder::new(self.core.clone(), sysplex, system, path)
    }

    /// # Examples
    ///
    /// Import system variables from a CSV file:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// zosmf.variables()
    ///     .import("TESTPLEX", "TESTNODE", "/u/testuser/variables.csv")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn import(&self, sysplex: &str, system: &str, path: &str) -> Result<(), Error> {
        ImportBuilder::new(self.core.clone(), sysplex, system, path)
            .build()
            .await
    }

    /// # Examples
    ///
    /// List all system variables on the local system:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let variables = zosmf.variables().list().build().await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// List all system variables on a named system:
    /// ```
    /// # use z_osmf::variables::list::SystemId;
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let system_id = SystemId::Named {
    ///     sysplex: "TESTPLEX".to_string(),
    ///     system: "TESTNODE".to_string(),
    /// };
    ///
    /// let variables = zosmf.variables()
    ///     .list()
    ///     .system_id(system_id)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(&self) -> VariablesBuilder<Variables> {
        VariablesBuilder::new(self.core.clone())
    }

    /// # Examples
    ///
    /// List all system symbols on the local system:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let symbols = zosmf.variables().symbols().build().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn symbols(&self) -> SymbolsBuilder<Symbols> {
        SymbolsBuilder::new(self.core.clone())
    }
}
