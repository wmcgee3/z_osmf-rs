#[path = "_setup/mod.rs"]
mod _setup;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _zosmf = _setup::get_zosmf().await?;

    Ok(())
}
