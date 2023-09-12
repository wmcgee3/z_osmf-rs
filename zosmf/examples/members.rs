use zosmf::datasets::members::BaseMembers;

#[path = "setup/setup.rs"]
mod setup;

use setup::get_zosmf;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let zosmf = get_zosmf().await?;

    let pds_members = zosmf
        .datasets()
        .list_members("SOME.PDS.NAME")
        .attributes_base()
        .build()
        .await?;

    match pds_members.items() {
        BaseMembers::FixedOrVariable(_) => println!("PDS has fixed or variable formatting!"),
        BaseMembers::Undefined(_) => println!("PDS has undefined formatting!"),
    }

    Ok(())
}
