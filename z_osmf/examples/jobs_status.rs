#[path = "_setup/mod.rs"]
mod _setup;

use z_osmf::jobs::JobIdentifier;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let jobs_client = _setup::get_zosmf().await?.jobs();

    let identifier = JobIdentifier::NameId("BLSJPRMI".into(), "STC00052".into());

    let jobs_status = jobs_client.status(identifier).exec_data().build().await?;

    println!("{:#?}", jobs_status);

    Ok(())
}
