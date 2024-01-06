#[path = "_setup/mod.rs"]
mod _setup;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let datasets_client = _setup::get_zosmf().await?.datasets();

    let read_member = datasets_client
        .read("SYS1.PARMLIB")
        .member("SMFPRM00")
        .build()
        .await?;

    println!("{}", read_member.data());

    let read_dataset = datasets_client.read("JIAHJ.REST.SRVMP").build().await?;

    println!("{}", read_dataset.data());

    Ok(())
}
