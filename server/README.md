
### protobuf 的使用

#### 主要文件：

- proto/hello.proto
- grpc/client.rs
- build.rs

#### 步骤：

1. 先项目根目录下运行编译 proto 内容

```shell
cargo build
```

2. grpc/client.rs 中定义连接 client 的方法
3. 启动 go 写的 grpc 的服务端，项目代码在 golang-about/protobuf-test
4. 运行 grpc/client.rs 里的测试用例
