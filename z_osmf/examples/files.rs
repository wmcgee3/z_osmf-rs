use z_osmf::files::list::DaysLastModified;

#[path = "_setup/mod.rs"]
mod _setup;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let zosmf = _setup::get_zosmf().await?;

    let file_list = zosmf
        .files()
        .list("/home/mcgeew2")
        .depth(1)
        .modified_days(DaysLastModified::MoreThan(1))
        .build()
        .await?;

    println!("{:#?}", file_list.items());

    Ok(())
}
