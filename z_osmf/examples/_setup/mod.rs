pub async fn get_zosmf() -> anyhow::Result<z_osmf::ZOsmf> {
    let _ = dotenvy::dotenv();

    let base_url = std::env::var("z_osmf_BASE_URL")?;
    let username = std::env::var("z_osmf_USERNAME")?;
    let password = std::env::var("z_osmf_PASSWORD")?;

    let mut client_builder = reqwest::Client::builder();

    if let Ok(cert_path) = std::env::var("z_osmf_CERT_PATH") {
        let text = std::fs::read_to_string(cert_path)?;
        let cert = reqwest::Certificate::from_pem(text.as_bytes())?;

        client_builder = client_builder.use_rustls_tls().add_root_certificate(cert);
    }

    let zosmf = z_osmf::ZOsmf::new(client_builder, base_url)?;

    zosmf.login(&username, password).await?;

    Ok(zosmf)
}
