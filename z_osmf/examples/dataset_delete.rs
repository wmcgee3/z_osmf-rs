#[path = "_setup/mod.rs"]
mod _setup;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let datasets_client = _setup::get_z_osmf().await?.datasets();

    let delete_dataset = datasets_client
        .delete("JIAHJ.REST.TEST.DATASET")
        .build()
        .await?;

    println!("{:#?}", delete_dataset);

    let delete_uncataloged = datasets_client
        .delete("JIAHJ.REST.TEST.DATASET2")
        .volume("ZMF046")
        .build()
        .await?;

    println!("{:#?}", delete_uncataloged);

    let delete_member = datasets_client
        .delete("JIAHJ.REST.TEST.PDS")
        .member("MEMBER01")
        .build()
        .await?;

    println!("{:#?}", delete_member);

    let delete_uncataloged_member = datasets_client
        .delete("JIAHJ.REST.TEST.PDS.UNCAT")
        .member("MEMBER01")
        .volume("ZMF046")
        .build()
        .await?;

    println!("{:#?}", delete_uncataloged_member);

    Ok(())
}
