mod shim;
use shim::load_package_manage_shim;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    load_package_manage_shim("npm", "npx").await?;
    Ok(())
}
