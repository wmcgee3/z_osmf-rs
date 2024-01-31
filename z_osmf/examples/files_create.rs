#[path = "_setup/mod.rs"]
mod _setup;

use z_osmf::files::FileType;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let files_client = _setup::get_zosmf().await?.files();

    let file_create = files_client
        .create("/u/jiahj/text3.txt")
        .file_type(FileType::File)
        .mode("RWXRW-RW-")
        .build()
        .await?;
    println!("{}", file_create.transaction_id());

    Ok(())
}
