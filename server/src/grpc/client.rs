use crate::grpc::client;
use crate::pb;
use crate::pb::HelloRequest;
use crate::pb::hello_service_client::HelloServiceClient;
use std::sync::OnceLock;
use std::thread;
use tonic::transport::Channel;

static GRPC_CLIENT: OnceLock<HelloServiceClient<Channel>> = OnceLock::new();

pub async fn get_client() -> Result<HelloServiceClient<Channel>, Box<dyn std::error::Error>> {
    match GRPC_CLIENT.get() {
        Some(client) => Ok(client.clone()),
        None => {
            println!("init client");
            let mut client =
                pb::hello_service_client::HelloServiceClient::connect("http://localhost:50051")
                    .await?;
            GRPC_CLIENT
                .set(client.clone())
                .expect("TODO: panic message");

            Ok(client)
        }
    }
}

#[tokio::test]
async fn test_grpc_client() {
    let names = ["jack", "john", "bob", "judy"];
    let mut handlers = vec![];
    for name in names {
        let item = name.to_string().clone();
        let handler = thread::spawn(move || async {
            let mut client = client::get_client().await.expect("client failed");
            let request = tonic::Request::new(HelloRequest { name: item });
            let response = client.say_hello(request).await.unwrap();
            println!("Response from client: {:?}", response.into_inner().message);
        });
        handlers.push(handler);
    }
    for handler in handlers {
        handler.join().unwrap().await;
    }
}
