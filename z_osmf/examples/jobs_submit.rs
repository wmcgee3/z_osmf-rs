#[path = "_setup/mod.rs"]
mod _setup;

use z_osmf::jobs::submit::{JclData, JclSource};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let jobs_client = _setup::get_zosmf().await?.jobs();

    let jcl = r#"//TESTJOBX JOB (),MSGCLASS=H
// EXEC PGM=IEFBR14
"#;

    let job_data = jobs_client
        .submit(JclSource::Jcl(JclData::Text(jcl.into())))
        .build()
        .await?;

    println!("{:#?}", job_data);

    Ok(())
}
