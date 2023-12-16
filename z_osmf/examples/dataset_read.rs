#[path = "_setup/mod.rs"]
mod _setup;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let z_osmf = _setup::get_z_osmf().await?;

    let read_member = z_osmf
        .datasets
        .read("SYS1.PARMLIB")
        .member("SMFPRM00")
        .build()
        .await?;

    println!("{}", read_member.data());

    let read_dataset = z_osmf.datasets.read("JIAHJ.REST.SRVMP").build().await?;

    println!("{}", read_dataset.data());

    Ok(())
}
