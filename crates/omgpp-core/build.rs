fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=proto/general-message.proto");

    println!("omgpp-core build started");

    // Use this in build.rs
    protobuf_codegen::Codegen::new()
        // Use `protoc` parser, optional.
        .protoc()
        // Use `protoc-bin-vendored` bundled protoc command, optional.
        // .protoc_path(&protoc_bin_vendored::protoc_bin_path().unwrap())
        // All inputs and imports from the inputs must reside in `includes` directories.
        .includes(&["proto"])
        // Inputs must reside in some of include paths.
        .input("proto/general-message.proto")
        // Specify output directory relative to Cargo output directory.
        .cargo_out_dir("proto")
        .run_from_script();

    let core_result = csbindgen::Builder::default()
        .input_extern_file("src/ffi.rs")
        .input_extern_file("src/lib.rs")
        .always_included_types(["EndpointFFI", "UuidFFI","ConnectionState"])
        .csharp_class_name("OmgppCoreNative")
        .csharp_class_accessibility("public")
        .csharp_namespace("OmgppNative")
        .generate_csharp_file("../../generated/csharp/Core.g.cs");
    if let Err(error) = core_result {
        panic!("Failed to generate file: {}", &error.to_string());
    }
}
