#[path = "_setup/mod.rs"]
mod _setup;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let z_osmf = _setup::get_z_osmf().await?;

    let list_members = z_osmf.datasets.list_members("SYS1.PROCLIB").build().await?;

    println!("{:#?}", list_members);

    let list_members_base = z_osmf
        .datasets
        .list_members("SYS1.PROCLIB")
        .attributes_base()
        .build()
        .await?;

    println!("{:#?}", list_members_base);

    Ok(())
}
