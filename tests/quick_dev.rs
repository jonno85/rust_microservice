#![allow(unused)]

use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8090")?;

    // hc.do_get("/hello?name=Jen").await?.print().await?;
    hc.do_get("/hello2/Mike").await?.print().await?;

    // hc.do_get("/src/main.rs").await?.print().await?;
    let req_login = hc.do_post(
        "api/login", 
        json!({
            "username": "demo1",
            "pwd": "welcome"
        }));

    Ok(())
}
