use z_osmf::files::list::{FileFilter, FileSize, ListFileType};

#[path = "_setup/mod.rs"]
mod _setup;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let zosmf = _setup::get_zosmf().await?;

    let file_list = zosmf
        .files()
        .list("/home/mcgeew2")
        .depth(1)
        .modified_days(FileFilter::GreaterThan(1))
        .size(FileFilter::GreaterThan(FileSize::Megabytes(1)))
        .file_type(ListFileType::File)
        .build()
        .await?;

    println!("{:#?}", file_list.items());

    Ok(())
}
