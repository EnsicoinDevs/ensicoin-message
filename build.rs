#[cfg(feature = "grpc")]
fn main() {
    use prost_build::compile_protos;
    compile_protos(&["ensicoin-proto/node.proto"], &["ensicoin-proto"]).unwrap();
}

#[cfg(not(feature = "grpc"))]
fn main() {}
