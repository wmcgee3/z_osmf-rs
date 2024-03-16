#[path = "_setup/mod.rs"]
mod _setup;

use anyhow::Context;
use rand::seq::IteratorRandom;
use z_osmf::jobs::files::Id;
use z_osmf::jobs::Identifier;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let jobs_client = _setup::get_zosmf().await?.jobs();

    let _ = dotenvy::dotenv_override();
    let username = std::env::var("ZOSMF_USERNAME")?;

    let my_jobs = jobs_client.list().owner(username).build().await?;
    let job_identifiers: Vec<Identifier> = my_jobs.items().iter().map(|j| j.identifier()).collect();
    println!(
        "Job Identifiers:\n{}\n",
        job_identifiers
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    );

    let mut rng = rand::thread_rng();
    let random_job_identifier = job_identifiers
        .iter()
        .choose(&mut rng)
        .context("failed to randomly select a job identifier")?;
    println!(
        "Random Job Identifier: {}",
        random_job_identifier.to_string()
    );

    let jcl_read = jobs_client
        .read_file(random_job_identifier.clone(), Id::Jcl)
        .build()
        .await?;
    println!("Random job JCL:\n{}\n", jcl_read.data());

    Ok(())
}
