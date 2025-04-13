### kafka 的使用

- docker 安装 kafka
```yml
version: '3.8'

services:
  kafka:
    image: bitnami/kafka:latest
    container_name: kafka
    ports:
      - "9092:9092"
    environment:
      KAFKA_CFG_NODE_ID: 1
      KAFKA_CFG_PROCESS_ROLES: controller,broker
      KAFKA_CFG_CONTROLLER_QUORUM_VOTERS: 1@kafka:9093
      KAFKA_CFG_CONTROLLER_LISTENER_NAMES: CONTROLLER
      KAFKA_CFG_LISTENER_SECURITY_PROTOCOL_MAP: CONTROLLER:PLAINTEXT,PLAINTEXT:PLAINTEXT
      KAFKA_CFG_LISTENERS: PLAINTEXT://:9092,CONTROLLER://:9093
      KAFKA_CFG_ADVERTISED_LISTENERS: PLAINTEXT://localhost:9092
    restart: unless-stopped
```

- 启动 kafka 服务

```shell
docker-compose up -d

# 查看日志
docker-compose logs -f
```

- producer kafka 消息
```http request
GET http://localhost:3000/producer_kafka_message
```

- consumer kafka 消费消息（代码中设置了6000ms的timeout）
```http request
http://localhost:3000/consumer_kafka_message
```

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
3. 启动 go 写的 grpc 的服务端，项目代码在 [golang-about/protobuf-test](https://github.com/haokur/golang-about/tree/main/protobuf-test)
4. 运行 grpc/client.rs 里的测试用例
