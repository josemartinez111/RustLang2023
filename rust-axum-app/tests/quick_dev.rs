// FILE: tests/quick_dev.rs
// ___________________________________________________________

use anyhow::Result;
use serde_json::json;
use rust_axum_app::utils::utilities::Void;

// ___________________________________________________________

#[tokio::test]
async fn quick_dev() -> Result<Void> {
  let host = httpc_test::new_client("http://localhost:8080")?;
  
  host.do_get("/health-check2/Jesse").await?.print().await?;
  host.do_get("/src/main.rs").await?.print().await?;
  
  let req_login = host.do_post(
    "/api/login",
    json!({
      "username":"demo1",
      "pwd":"welcome"
    })
  );
  
  req_login.await?.print().await?;
  Ok(())
}
// ___________________________________________________________