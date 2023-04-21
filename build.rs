fn main() {
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir("src/google")  // you can change the generated code's location
        .compile(
            &["proto/googleapis/google/pubsub/v1/pubsub.proto"],
            &["proto/googleapis"], // specify the root location to search proto dependencies
        ).unwrap();
    tonic_build::compile_protos("proto/helloworld/helloworld.proto").unwrap();
    tonic_build::compile_protos("proto/echo/echo.proto").unwrap();
}
// 注意:
// 若自定义生成代码所存放的目录, 那么需要程先手动创建该目录.
// 使用自定义目录后需要使用 tonic::include_proto! 的替代方案.
