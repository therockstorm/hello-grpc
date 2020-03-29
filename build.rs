fn main() -> Result<(), Box<dyn std::error::Error>> {
  tonic_build::compile_protos("proto/hello_api.proto")?;
  Ok(())
}
