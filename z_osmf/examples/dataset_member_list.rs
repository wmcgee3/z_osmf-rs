#[path = "_setup/mod.rs"]
mod _setup;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let datasets_client = _setup::get_zosmf().await?.datasets();

    let list_members = datasets_client.list_members("SYS1.PROCLIB").build().await?;

    println!("{:#?}", list_members);

    let list_members_base = datasets_client
        .list_members("SYS1.PROCLIB")
        .attributes_base()
        .build()
        .await?;

    println!("{:#?}", list_members_base);

    Ok(())
}
