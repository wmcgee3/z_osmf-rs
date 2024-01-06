#[path = "_setup/mod.rs"]
mod _setup;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let files_client = _setup::get_zosmf().await?.files();

    files_client.delete("/u/jiahj/text.txt").build().await?;

    files_client.delete("/u/jiahj/testDir").build().await?;

    Ok(())
}
