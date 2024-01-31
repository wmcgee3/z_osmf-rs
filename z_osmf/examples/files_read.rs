#[path = "_setup/mod.rs"]
mod _setup;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let files_client = _setup::get_zosmf().await?.files();

    let file_read = files_client.read("/etc/inetd.conf").build().await?;
    println!("{}", file_read.data());

    Ok(())
}
