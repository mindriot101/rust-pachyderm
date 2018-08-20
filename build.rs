extern crate protoc_grpcio;

fn main() {
    let proto_root = "protos";
    println!("cargo:rerun-if-changed={}", proto_root);
    protoc_grpcio::compile_grpc_protos(
        &["auth.proto", "pfs.proto", "empty.proto", "wrappers.proto"],
        &[proto_root],
        "src/protos",
    ).expect("failed to compile gRPC definitions");
}
