use tonic_build::compile_protos;

#[test]
fn test_protobuf(){
    compile_protos("proto/hello.proto").unwrap();
}

