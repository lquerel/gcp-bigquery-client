fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_transport(false)
        .out_dir("src/google")
        .compile(
            &["googleapis/google/cloud/bigquery/storage/v1/storage.proto"],
            &["googleapis"],
        )?;
    Ok(())
}
