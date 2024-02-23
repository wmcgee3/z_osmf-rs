# z_osmf

The VERY work in progress Rust z/OSMF<sup>TM</sup> [^1] Client.

## Examples

List your datasets:
```rust no_run
#[tokio::main]
async fn main() -> Result<(), z_osmf::Error> {
    let client = reqwest::Client::new();
    let base_url = "https://mainframe.my-company.com";

    let zosmf = z_osmf::ZOsmf::new(client, base_url);
    zosmf.login("USERNAME", "PASSWORD").await?;

    let my_datasets = zosmf
        .datasets()
        .list("USERNAME")
        .build()
        .await?;

    for dataset in my_datasets.items().iter() {
        println!("{}", dataset.name());
    }

    Ok(())
}
```

List the files in your home directory:
```rust no_run
#[tokio::main]
async fn main() -> Result<(), z_osmf::Error> {
    let client = reqwest::Client::new();
    let base_url = "https://mainframe.my-company.com";

    let zosmf = z_osmf::ZOsmf::new(client, base_url);
    zosmf.login("USERNAME", "PASSWORD").await?;

    let my_files = zosmf
        .files()
        .list("/u/username")
        .build()
        .await?;

    for file in my_files.items().iter() {
        println!("{}", file.name());
    }

    Ok(())
}
```

List all active jobs:
```rust no_run
#[tokio::main]
async fn main() -> Result<(), z_osmf::Error> {
    let client = reqwest::Client::new();
    let base_url = "https://mainframe.my-company.com";

    let zosmf = z_osmf::ZOsmf::new(client, base_url);
    zosmf.login("USERNAME", "PASSWORD").await?;

    let active_jobs = zosmf
        .jobs()
        .list()
        .owner("*")
        .active_only()
        .build()
        .await?;

    for job in active_jobs.items().iter() {
        println!("{}", job.name());
    }

    Ok(())
}
```

---

[^1]: z/OSMF<sup>TM</sup>, z/OS<sup>TM</sup>, and the lowercase letter z<sup>TM</sup> (probably) are trademarks owned by International Business Machines Corporation ("IBM").
This crate is not approved, endorsed, acknowledged, or even tolerated by IBM.
(Please don't sue me, Big Blue)
