#[path = "_setup/mod.rs"]
mod _setup;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let zosmf = _setup::get_zosmf().await?;

    let list_members = zosmf.datasets.list_members("SYS1.PROCLIB").build().await?;

    println!("{:#?}", list_members);

    let list_members_base = zosmf
        .datasets
        .list_members("SYS1.PROCLIB")
        .attributes_base()
        .build()
        .await?;

    println!("{:#?}", list_members_base);

    Ok(())
}
