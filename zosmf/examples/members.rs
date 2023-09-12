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
        BaseMembers::FixedOrVariable(fov) => {
            println!(
                "My Fixed or Variable Format Members:\n\n{}\n",
                fov.iter().map(|m| m.name()).collect::<Vec<_>>().join("\n")
            );
        }
        BaseMembers::Undefined(u) => {
            println!(
                "My Undefined Formatting Members:\n\n{}\n",
                u.iter().map(|m| m.name()).collect::<Vec<_>>().join("\n")
            );
        }
    }

    Ok(())
}
