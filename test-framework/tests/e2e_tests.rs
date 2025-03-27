use eyre::Result;
use test_framework::components::{Components, ContractVersion, ImageTags};
#[tokio::test(flavor = "multi_thread")]
async fn dummy_test() -> Result<()> {
    let tags = ImageTags::default();
    let components = Components::new(None, tags).await?;
    Ok(())
}
