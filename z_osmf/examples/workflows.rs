#[path = "_setup/mod.rs"]
mod _setup;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let zosmf = _setup::get_zosmf().await?;

    let workflows = zosmf.workflows().list().build().await?;

    println!("{:#?}", workflows.items());

    for workflow in workflows.items().iter() {
        match zosmf
            .workflows()
            .properties(workflow.key())
            .steps()
            .build()
            .await
        {
            Ok(properties) => {
                println!("{:#?}", properties);
            }
            Err(err) => {
                println!("{}", workflow.key());

                return Err(err.into());
            }
        }
    }

    // let properties = zosmf
    //     .workflows()
    //     .properties("99731a5d-25f7-4d7f-b80c-8d5e580e05ff")
    //     .steps()
    //     .build()
    //     .await?;

    // println!("{:#?}", properties);

    Ok(())
}
