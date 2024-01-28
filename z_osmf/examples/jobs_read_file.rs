#[path = "_setup/mod.rs"]
mod _setup;

use z_osmf::jobs::{Identifier, JobFileID};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let jobs_client = _setup::get_zosmf().await?.jobs();

    let job_identifier = Identifier::NameId("TESTJOBJ".into(), "JOB00023".into());

    let job_file = jobs_client
        .read_file(job_identifier.clone(), JobFileID::ID(1))
        .build()
        .await?;

    println!("{}", job_file.data);

    // read the first 250 records
    let job_file = jobs_client
        .read_file(job_identifier.clone(), JobFileID::ID(8))
        .record_range("0-249")
        .build()
        .await?;

    println!("{}", job_file.data);

    // read JCL
    let job_file = jobs_client
        .read_file(job_identifier, JobFileID::JCL)
        .build()
        .await?;

    println!("{}", job_file.data);

    Ok(())
}
