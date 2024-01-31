#[path = "_setup/mod.rs"]
mod _setup;

use z_osmf::jobs::JobIdentifier;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let jobs_client = _setup::get_zosmf().await?.jobs();

    let identifier = JobIdentifier::NameId("TESTJOB1".into(), "JOB00023".into());

    let job_files = jobs_client.list_files(identifier).build().await?;

    println!("{:#?}", job_files.items());

    Ok(())
}
