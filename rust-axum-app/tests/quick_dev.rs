// FILE: tests/quick_dev.rs
// ___________________________________________________________

use anyhow::Result;
// ___________________________________________________________

#[tokio::test]
async fn quick_dev() -> Result<()> {
  let host = httpc_test::new_client("http://localhost:8080")?;
  host.do_get("/health-check2/Jesse").await?.print().await?;
  Ok(())
}
// ___________________________________________________________