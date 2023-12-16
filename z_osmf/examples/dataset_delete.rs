#[path = "_setup/mod.rs"]
mod _setup;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let z_osmf = _setup::get_z_osmf().await?;

    let delete_dataset = z_osmf
        .datasets
        .delete("JIAHJ.REST.TEST.DATASET")
        .build()
        .await?;

    println!("{:#?}", delete_dataset);

    let delete_uncataloged = z_osmf
        .datasets
        .delete("JIAHJ.REST.TEST.DATASET2")
        .volume("ZMF046")
        .build()
        .await?;

    println!("{:#?}", delete_uncataloged);

    let delete_member = z_osmf
        .datasets
        .delete("JIAHJ.REST.TEST.PDS")
        .member("MEMBER01")
        .build()
        .await?;

    println!("{:#?}", delete_member);

    let delete_uncataloged_member = z_osmf
        .datasets
        .delete("JIAHJ.REST.TEST.PDS.UNCAT")
        .member("MEMBER01")
        .volume("ZMF046")
        .build()
        .await?;

    println!("{:#?}", delete_uncataloged_member);

    Ok(())
}
