#[path = "_setup/mod.rs"]
mod _setup;

use anyhow::Context;
use rand::seq::IteratorRandom;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let zosmf = _setup::get_zosmf().await?;

    let username = std::env::var("ZOSMF_USERNAME")?;

    let dataset_list = zosmf.list_datasets(&username).build().await?;

    let dataset_names: Vec<&str> = dataset_list.items().iter().map(|d| d.name()).collect();
    println!("Datasets:\n{:#?}\n", dataset_names.join("\n"));

    let mut rng = rand::thread_rng();
    let random_dataset_name = dataset_list
        .items()
        .iter()
        .choose(&mut rng)
        .map(|d| d.name())
        .context("failed to select a random dataset!")?;

    println!("Randomly selected dataset: {}\n", random_dataset_name);

    let random_dataset_list = zosmf
        .list_datasets(random_dataset_name)
        .attributes_base()
        .max_items(1)
        .build()
        .await?;
    let random_dataset_attributes = random_dataset_list
        .items()
        .first()
        .context("failed to get first dataset!")?;

    let random_dataset_type = random_dataset_attributes.dataset_type();
    if random_dataset_type == Some("LIBRARY") || random_dataset_type == Some("PDS") {
        let member_list = zosmf
            .list_dataset_members(random_dataset_name)
            .build()
            .await?;
        let member_names: Vec<&str> = member_list.items().iter().map(|m| m.name()).collect();

        println!(
            "Partitioned dataset member names: \n{}\n",
            member_names.join("\n")
        );
    } else {
        let dataset_read = zosmf.read_dataset(random_dataset_name).build().await?;

        println!("Sequential dataset contents: \n{}\n", dataset_read.data());
    }

    Ok(())
}
