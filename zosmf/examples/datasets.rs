#[path = "setup/setup.rs"]
mod setup;

use setup::get_zosmf;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let zosmf = get_zosmf().await?;

    let username = std::env::var("ZOSMF_USERNAME")?;

    let my_datasets = zosmf.datasets().list(&username).build().await?;

    let my_dataset_names = my_datasets
        .items()
        .iter()
        .map(|ds| ds.name())
        .collect::<Vec<_>>()
        .join("\n");

    println!("My Datasets:\n\n{}\n", my_dataset_names);

    Ok(())
}
