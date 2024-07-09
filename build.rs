fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(storage)]
    tonic_build::configure()
        .build_server(false)
        .compile(
            &["googleapis/google/cloud/bigquery/storage/v1/storage.proto"],
            &["googleapis"],
        )?;
    Ok(())
}
