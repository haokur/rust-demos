
// 使用cargo build执行
// .rs 文件会被生成在 target/debug/build/xxx/out/ 目录下。
fn main() {
    tonic_build::compile_protos("proto/hello.proto").unwrap();
}
