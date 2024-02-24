#[path = "_setup/mod.rs"]
mod _setup;

use anyhow::Context;
use rand::seq::IteratorRandom;
use z_osmf::files::list::ListFileType;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let files_client = _setup::get_zosmf().await?.files();

    // change this to the path of your home directory
    let home_dir_path = "/u/username";

    let file_list = files_client
        .list(home_dir_path)
        .depth(1)
        .file_type(ListFileType::File)
        .build()
        .await?;
    let files_names: Vec<&str> = file_list.items().iter().map(|f| f.name()).collect();
    println!("Files:\n{}\n", files_names.join("\n"));

    let mut rng = rand::thread_rng();
    let random_file_name = files_names
        .iter()
        .choose(&mut rng)
        .context("failed to select a random file")?;
    println!("Randomly selected file: {}", random_file_name);

    let random_file_read = files_client
        .read(&format!("{}/{}", home_dir_path, random_file_name))
        .build()
        .await?;
    println!("Random file contents:\n{}", random_file_read.data());

    Ok(())
}
