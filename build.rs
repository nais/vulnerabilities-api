fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .out_dir("src/proto")
        .compile_protos(&["proto/helloworld/helloworld.proto"],&["/proto/"])?;
    Ok(())
}