fn main() {
    //  println!("cargo:rustc-link-lib=Advapi32");
     let mut config = prost_build::Config::new();
    config.out_dir("./src/");

    config.type_attribute(".", "#[derive(::serde::Serialize, ::serde::Deserialize)]");
    config.btree_map(&[".my_messages"]);

    config
        .compile_protos(
            &[ "src/packet.proto"],
            &["src"],
        )
        .unwrap();
    
}
