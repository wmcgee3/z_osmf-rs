#[path = "_setup/mod.rs"]
mod _setup;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let datasets_client = _setup::get_zosmf().await?.datasets();

    let list_datasets = datasets_client.list("IBMUSER.CONFIG.*").build().await?;

    println!("{:#?}", list_datasets);

    let list_datasets_base = datasets_client
        .list("**")
        .volume("PEVTS2")
        .attributes_base()
        .build()
        .await?;

    println!("{:#?}", list_datasets_base);

    Ok(())
}
