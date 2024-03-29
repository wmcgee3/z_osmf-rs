#[path = "_setup/mod.rs"]
mod _setup;

use anyhow::Context;
use rand::seq::IteratorRandom;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let zosmf = _setup::get_zosmf().await?;

    let _ = dotenvy::dotenv_override();
    let username = std::env::var("ZOSMF_USERNAME")?;

    let datasets_client = zosmf.datasets();

    let dataset_list = datasets_client.list(&username).build().await?;

    let dataset_names: Vec<&str> = dataset_list.items().iter().map(|d| d.name()).collect();
    println!("Datasets:\n{}\n", dataset_names.join("\n"));

    let mut rng = rand::thread_rng();
    let random_dataset_name = dataset_list
        .items()
        .iter()
        .choose(&mut rng)
        .map(|d| d.name())
        .context("failed to select a random dataset!")?;

    println!("Randomly selected dataset: {}\n", random_dataset_name);

    let random_dataset_list = datasets_client
        .list(random_dataset_name)
        .attributes_base()
        .max_items(1)
        .build()
        .await?;
    let random_dataset_attributes = random_dataset_list
        .items()
        .first()
        .context("failed to get first dataset!")?;

    match random_dataset_attributes.organization() {
        Some(dsorg) if dsorg.starts_with("PO") => {
            let member_list = datasets_client.members(random_dataset_name).build().await?;
            let member_names: Vec<&str> = member_list.items().iter().map(|m| m.name()).collect();

            println!(
                "Partitioned dataset member names: \n{}\n",
                member_names.join("\n")
            );
        }
        Some(_) => {
            let dataset_read = datasets_client.read(random_dataset_name).build().await?;

            println!("Sequential dataset contents: \n{}\n", dataset_read.data());
        }
        _ => println!("Unknown dataset type, doing nothing!"),
    }

    Ok(())
}
