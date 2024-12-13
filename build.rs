use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = &PathBuf::from(env::var("OUT_DIR").unwrap());

    /*
    tonic_build::configure()
        .file_descriptor_set_path("src/proto/helloworld_descriptor.bin")
        .out_dir("src/proto")
        .compile_protos(&["proto/helloworld/helloworld.proto"], &["/proto/"])?;
    Ok(())

     */
    tonic_build::configure()
        .compile_protos(&["proto/helloworld/helloworld.proto"], &["proto"])
        .unwrap();

    Ok(())
}
