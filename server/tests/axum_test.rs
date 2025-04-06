use std::error::Error;
use serde_json::Value;

#[tokio::test]
async fn send_post_data() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    let v: Value = serde_json::from_str(data)?;
    let json_body = serde_json::to_string(&v).unwrap();

    let res = client
        .post("http://localhost:3000/post_data")
        .header("content-type", "application/json")
        .body(json_body)
        .send()
        .await?;

    println!("Response: {:?}", res);

    Ok(())
}
