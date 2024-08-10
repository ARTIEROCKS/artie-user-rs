fn main() {
    tonic_build::configure()
        .build_server(true)
        .file_descriptor_set_path("proto/user_descriptor.bin") 
        .compile(&["proto/user.proto"], &["proto"])
        .unwrap();
}