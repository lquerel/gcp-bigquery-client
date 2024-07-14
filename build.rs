fn main() -> Result<(), Box<dyn std::error::Error>> {
    // The gRPC Storage Write API is vendored in `src/google` to avoid
    // depending on protoc in GitHub Actions.
    //
    // To regenerate the gRPC Storage Write API from the proto file:
    // - Comment the build_transport(false) line below.
    // - Run `cargo build` to regenerate the API.
    // - Uncomment the build_transport(false) line below.
    // - Commit the changes.
    tonic_build::configure()
        .build_transport(false)
        .build_server(false)
        .out_dir("src/google")
        .compile(
            &["googleapis/google/cloud/bigquery/storage/v1/storage.proto"],
            &["googleapis"],
        )?;
    Ok(())
}