mod shim;

use shim::{exec_default, node_exec_strict};
use snm_config::parse_snm_config;
use snm_core::traits::manage::ManageTrait;
use snm_current_dir::current_dir;
use snm_package_json::parse_package_json;
use snm_package_manager::snm_package_manager::SnmPackageManager;
use snm_utils::exec::exec_cli;
use std::ops::Not;
const BIN_NAME: &str = "pnpm";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let dir = current_dir()?;

    let snm_config = parse_snm_config(&dir)?;

    let snm_node: Box<dyn ManageTrait> =
        Box::new(SnmPackageManager::from_prefix(BIN_NAME, snm_config.clone()));
    if let Some(package_manager) = parse_package_json(&dir).and_then(|x| x.package_manager) {
        if package_manager.name.unwrap() != BIN_NAME {
            panic!("NoPackageManager");
        }

        if let Ok(x) = node_exec_strict(
            &snm_node,
            snm_config.clone(),
            BIN_NAME,
            package_manager.version,
        )
        .await
        {
            println!("严格模式 {:?}", x);
            let args: Vec<String> = std::env::args().skip(1).collect();
            exec_cli(x, args);

            return Ok(());
        }

        if snm_config.get_strict().not() {
            if let Ok(sss) = exec_default(&snm_node, BIN_NAME).await {
                println!("默认版本 {:?}", sss);
            }
        }
    }

    Ok(())
}
