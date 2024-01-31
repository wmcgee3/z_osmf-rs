#[path = "_setup/mod.rs"]
mod _setup;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let files_client = _setup::get_zosmf().await?.files();

    let files_list = files_client.list("/usr").build().await?;
    println!("{:#?}", files_list.items());

    let files_list = files_client.list("/u/ibmuser/myFile.txt").build().await?;
    println!("{:#?}", files_list.items());

    let files_list = files_client
        .list("/usr/include")
        .name("f*.h")
        .build()
        .await?;
    println!("{:#?}", files_list.items());

    Ok(())
}
