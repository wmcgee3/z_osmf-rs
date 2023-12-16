#[path = "_setup/mod.rs"]
mod _setup;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let z_osmf = _setup::get_z_osmf().await?;

    let list_datasets = z_osmf.datasets.list("IBMUSER.CONFIG.*").build().await?;

    println!("{:#?}", list_datasets);

    let list_datasets_base = z_osmf
        .datasets
        .list("**")
        .volume("PEVTS2")
        .attributes_base()
        .build()
        .await?;

    println!("{:#?}", list_datasets_base);

    Ok(())
}
