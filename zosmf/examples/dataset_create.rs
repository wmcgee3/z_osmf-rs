#[path = "_setup/mod.rs"]
mod _setup;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let zosmf = _setup::get_zosmf().await?;

    let create_dataset = zosmf
        .datasets
        .create("JIAHJ.REST.TEST.NEWDS")
        .volume("zmf046")
        .device_type("3390")
        .organization("PS")
        .space_allocation_unit("TRK")
        .primary_space(10)
        .secondary_space(5)
        .average_block_size(500)
        .record_format("FB")
        .block_size(400)
        .logical_record_length(80)
        .build()
        .await?;

    println!("{:#?}", create_dataset);

    let create_pds = zosmf
        .datasets
        .create("JIAHJ.REST.TEST.NEWDS02")
        .volume("zmf046")
        .device_type("3390")
        .organization("PO")
        .space_allocation_unit("TRK")
        .primary_space(10)
        .secondary_space(5)
        .directory_blocks(10)
        .average_block_size(500)
        .record_format("FB")
        .block_size(400)
        .logical_record_length(80)
        .build()
        .await?;

    println!("{:#?}", create_pds);

    let create_pdse = zosmf
        .datasets
        .create("JIAHJ.REST.TEST.NEWDS02")
        .volume("zmf046")
        .device_type("3390")
        .organization("PO")
        .space_allocation_unit("TRK")
        .primary_space(10)
        .secondary_space(5)
        .directory_blocks(10)
        .average_block_size(500)
        .record_format("FB")
        .block_size(400)
        .logical_record_length(80)
        .dataset_type("LIBRARY")
        .build()
        .await?;

    println!("{:#?}", create_pdse);

    Ok(())
}
