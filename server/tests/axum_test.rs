use serde_json::Value;
use std::error::Error;

#[tokio::test]
async fn test_root() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let res = client.get("http://localhost:3000/").send().await?;
    let body = res.text().await?;

    assert_eq!(body, "Hello,This Server Root!");

    Ok(())
}

#[tokio::test]
async fn test_put_some() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let res = client.put("http://localhost:3000/put_some").send().await?;
    let body = res.text().await?;

    assert_eq!(body, "can't do put");

    Ok(())
}

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

    let body = res.text().await?;

    println!("Response body is : {:?}", body);

    Ok(())
}
