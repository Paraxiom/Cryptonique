// build.rs
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../QuantumTimeSandwich/QuantumTimeSandwich/grpc/proto/quantum_encryption_service.proto")?;
    Ok(())
}