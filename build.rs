fn main() -> Result<(), Box<dyn std::error::Error>> {
    // The gRPC Storage Write API is vendored in `src/google` to avoid
    // depending on protoc in GitHub Actions.
    //
    // To regenerate the gRPC Storage Write API from the proto file:
    // - Uncomment the following lines.
    // - Run `cargo build` to regenerate the API.
    // - Comment the following lines.
    // - Commit the changes.

    // tonic_prost_build::configure()
    //     .build_server(false)
    //     .out_dir("src/google")
    //     .compile_protos(
    //         &["googleapis/google/cloud/bigquery/storage/v1/storage.proto"],
    //         &["googleapis"],
    //     )?;
    Ok(())
}
