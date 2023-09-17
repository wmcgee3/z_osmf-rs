#[path = "_setup/mod.rs"]
mod _setup;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let zosmf = _setup::get_zosmf().await?;

    let delete_dataset = zosmf
        .datasets
        .delete("JIAHJ.REST.TEST.DATASET")
        .build()
        .await?;

    println!("{:#?}", delete_dataset);

    let delete_uncataloged = zosmf
        .datasets
        .delete("JIAHJ.REST.TEST.DATASET2")
        .volume("ZMF046")
        .build()
        .await?;

    println!("{:#?}", delete_uncataloged);

    let delete_member = zosmf
        .datasets
        .delete("JIAHJ.REST.TEST.PDS")
        .member("MEMBER01")
        .build()
        .await?;

    println!("{:#?}", delete_member);

    let delete_uncataloged_member = zosmf
        .datasets
        .delete("JIAHJ.REST.TEST.PDS.UNCAT")
        .member("MEMBER01")
        .volume("ZMF046")
        .build()
        .await?;

    println!("{:#?}", delete_uncataloged_member);

    Ok(())
}
