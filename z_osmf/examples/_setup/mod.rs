pub async fn get_zosmf() -> anyhow::Result<z_osmf::ZOsmf> {
    let _ = dotenvy::dotenv_override();

    let base_url = std::env::var("ZOSMF_BASE_URL")?;
    let username = std::env::var("ZOSMF_USERNAME")?;
    let password = std::env::var("ZOSMF_PASSWORD")?;

    let mut client_builder = reqwest::Client::builder();

    if let Ok(cert_path) = std::env::var("ZOSMF_CERT_PATH") {
        let text = std::fs::read_to_string(cert_path)?;
        let cert = reqwest::Certificate::from_pem(text.as_bytes())?;

        client_builder = client_builder.use_rustls_tls().add_root_certificate(cert);
    }

    let zosmf = z_osmf::ZOsmf::new(client_builder.build()?, base_url);

    zosmf.login(username, password).await?;

    Ok(zosmf)
}
