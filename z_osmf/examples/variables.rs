#[path = "_setup/mod.rs"]
mod _setup;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let zosmf = _setup::get_zosmf().await?;

    let variables = zosmf.system_variables().list().build().await?;

    println!("{:#?}", variables);

    let symbols = zosmf.system_variables().symbols().build().await?;

    println!("{:#?}", symbols);

    Ok(())
}
