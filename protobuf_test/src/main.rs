use hello::hello_service_server::{HelloService, HelloServiceServer};
use hello::{HelloRequest, HelloResponse};
use tonic::{Request, Response, Status, transport::Server};

pub mod hello {
    tonic::include_proto!("hello");
}

#[derive(Default)]
pub struct MyHelloService;

#[tonic::async_trait]
impl HelloService for MyHelloService {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply = HelloResponse {
            message: format!("Hello, {}!", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }
}

// 启动服务端的异步任务
async fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let hello_service = MyHelloService::default();

    println!("Server listening on {}", addr);

    // 启动 gRPC 服务端
    Server::builder()
        .add_service(HelloServiceServer::new(hello_service))
        .serve(addr)
        .await?;

    Ok(())
}

// 客户端请求 gRPC 服务
async fn start_client() -> Result<(), Box<dyn std::error::Error>> {
    // 客户端连接到服务端
    let mut client =
        hello::hello_service_client::HelloServiceClient::connect("http://localhost:50051").await?;

    // 构建请求
    let request = tonic::Request::new(HelloRequest {
        name: "Rustacean".into(),
    });

    // 调用服务端的 SayHello 方法
    let response = client.say_hello(request).await?;

    // 打印响应
    println!("Response from server: {:?}", response);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 启动服务端
    tokio::spawn(async {
        if let Err(e) = start_server().await {
            eprintln!("Failed to start server: {:?}", e);
        }
    });

    // 客户端稍微延迟一些启动，以确保服务端已经准备好
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    // 启动客户端并与服务端通信
    if let Err(e) = start_client().await {
        eprintln!("Failed to start client: {:?}", e);
    }

    Ok(())
}
