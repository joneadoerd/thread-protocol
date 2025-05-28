use std::fs;
use std::path::PathBuf;

fn main() {
    // Set the directory where the .proto files are located
    let proto_dir = PathBuf::from("proto");

    // Collect all .proto files in the directory
    let protos: Vec<PathBuf> = fs::read_dir(&proto_dir)
        .unwrap()
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension()
                .map(|ext| ext == "proto")
                .unwrap_or(false)
        })
        .collect();

    // Set up the config for prost_build
    let mut config = prost_build::Config::new();
    config.out_dir("./src/");

    // Apply serde attributes to all types
    config.type_attribute(".", "#[derive(::serde::Serialize, ::serde::Deserialize)]");

    // Compile the .proto files
    config
        .compile_protos(&protos, &[proto_dir])
        .expect("Failed to compile .proto files");
}
