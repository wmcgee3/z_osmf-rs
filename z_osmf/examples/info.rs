#[path = "_setup/mod.rs"]
mod _setup;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let zosmf = _setup::get_zosmf().await?;

    let info = zosmf.info().await?;

    println!("{:#?}", info);

    Ok(())
}
