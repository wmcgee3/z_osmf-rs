#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let base_url = dotenvy::var("ZOSMF_BASE_URL")?;
    let username = dotenvy::var("ZOSMF_USERNAME")?;
    let password = dotenvy::var("ZOSMF_PASSWORD")?;

    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;

    let zosmf = z_osmf::ZOsmf::new(client, base_url);
    let tokens = zosmf.login(&username, password).await?;

    println!("{:#?}", tokens);

    let my_datasets = zosmf
        .datasets()
        .list(username)
        .attributes_vol()
        .build()
        .await?;

    println!("{:#?}", my_datasets);

    Ok(())
}
