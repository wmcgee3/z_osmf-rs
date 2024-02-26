#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = reqwest::Client::new();

    let _ = dotenvy::dotenv_override();
    let base_url = std::env::var("ZOSMF_BASE_URL")?;

    let zosmf = z_osmf::ZOsmf::new(client, base_url);

    let info = zosmf.info().await?;

    println!("z/OSMF API Version: {}", info.api_version());

    let plugin_names = info
        .plugins()
        .iter()
        .map(|p| p.default_name())
        .collect::<Vec<_>>();

    println!("Plugins:\n{}", plugin_names.join("\n"));

    Ok(())
}
