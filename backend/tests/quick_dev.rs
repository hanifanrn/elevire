use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8081")?;

    hc.do_get("/hello").await?.print().await?;
    println!("test executed!");

    Ok(())
}
