#[path = "_setup/mod.rs"]
mod _setup;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let jobs_client = _setup::get_zosmf().await?.jobs();

    let jobs_list = jobs_client
        .list()
        .owner("IBMUSER")
        .prefix("TESTJOB*")
        .exec_data()
        .build()
        .await?;

    println!("{:#?}", jobs_list.items());

    Ok(())
}
