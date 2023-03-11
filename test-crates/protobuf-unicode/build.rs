use protobuf_codegen::Codegen;

fn main() {
    // We generate descriptors twice: with pure rust codegen
    // and with codegen depending on `protoc` binary.
    // This is for demonstration purposes; in practice you'd need either of them.
    //
    // Note there's a third option: using `protoc` binary directly and `protoc-gen-rust`
    // plugin, this is a canonical way to generate protobuf sources.
    // This is not possible to do with Cargo (since Cargo cannot depend on binaries)
    // but can be used with some other build system.

    Codegen::new()
        .cargo_out_dir("protos")
        .input("protos/library.proto")
        .include("protos")
        .run_from_script();
}
