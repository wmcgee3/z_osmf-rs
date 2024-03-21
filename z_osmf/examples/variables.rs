#[path = "_setup/mod.rs"]
mod _setup;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let zosmf = _setup::get_zosmf().await?;

    let variables = zosmf.variables().list().build().await?;

    println!("{:#?}", variables);

    let symbols = zosmf.variables().symbols().build().await?;

    println!("{:#?}", symbols);

    Ok(())
}
