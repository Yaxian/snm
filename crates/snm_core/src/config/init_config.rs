use crate::model::SnmError;
use colored::*;
use std::{env, fs::create_dir_all, path::PathBuf};

static SNM_BASE_DIR_KEY: &str = "SNM_BASE_DIR";

static BIN_DIR_KEY: &str = "SNM_NODE_BIN_DIR";
static DOWNLOAD_DIR_KEY: &str = "SNM_DOWNLOAD_DIR";
static NODE_MODULES_DIR_KEY: &str = "SNM_NODE_MODULES_DIR";

pub static SNM_NPM_REGISTRY_HOST_KEY: &str = "SNM_NPM_REGISTRY_HOST";
pub static SNM_YARN_REGISTRY_HOST_KEY: &str = "SNM_YARN_REGISTRY_HOST_KEY";
pub static SNM_YARN_REPO_HOST_KEY: &str = "SNM_YARN_REPO_HOST_KEY";

static SNM_STRICT: &str = "SNM_STRICT";

pub struct SnmConfig {}

impl SnmConfig {
    pub fn new() -> Self {
        Self {}
    }

    pub fn init(&self) -> Result<(), SnmError> {
        self.init_strict();

        create_dir_all(self.get_base_dir_path_buf()?)?;
        create_dir_all(self.get_node_bin_dir_path_buf()?)?;
        create_dir_all(self.get_download_dir_path_buf()?)?;
        create_dir_all(self.get_node_modules_dir_path_buf()?)?;

        self.init_url_config();

        Ok(())
    }

    pub fn get_strict(&self) -> bool {
        let value = env::var(SNM_STRICT).unwrap_or(false.to_string());
        value.parse::<bool>().unwrap_or(false)
    }

    pub fn get_base_dir_path_buf(&self) -> Result<PathBuf, SnmError> {
        let home_dir = dirs::home_dir().ok_or(SnmError::GetHomeDirError)?;
        let base_dir_name = env::var(SNM_BASE_DIR_KEY).unwrap_or(".snm".to_string());
        let base_dir_path_buf = home_dir.join(base_dir_name);
        Ok(base_dir_path_buf)
    }

    pub fn get_node_bin_dir_path_buf(&self) -> Result<PathBuf, SnmError> {
        let base_dir = self.get_base_dir_path_buf()?;
        let node_bin_dir_name = env::var(BIN_DIR_KEY).unwrap_or("bin".to_string());
        let node_bin_dir_path_buf = base_dir.join(node_bin_dir_name);
        Ok(node_bin_dir_path_buf)
    }

    pub fn get_download_dir_path_buf(&self) -> Result<PathBuf, SnmError> {
        let base_dir = self.get_base_dir_path_buf()?;
        let download_dir_name = env::var(DOWNLOAD_DIR_KEY).unwrap_or("download".to_string());
        let download_dir_path_buf = base_dir.join(download_dir_name);
        Ok(download_dir_path_buf)
    }

    pub fn get_node_modules_dir_path_buf(&self) -> Result<PathBuf, SnmError> {
        let base_dir = self.get_base_dir_path_buf()?;
        let node_modules_dir_name =
            env::var(NODE_MODULES_DIR_KEY).unwrap_or("node_modules".to_string());
        let node_modules_dir_path_buf = base_dir.join(node_modules_dir_name);
        Ok(node_modules_dir_path_buf)
    }

    fn init_strict(&self) {
        if let Err(_) = env::var(SNM_STRICT) {
            env::set_var(SNM_STRICT, false.to_string());
        }
    }

    fn init_url_config(&self) {
        self.var(SNM_NPM_REGISTRY_HOST_KEY, "https://registry.npmjs.org");
        self.var(SNM_YARN_REGISTRY_HOST_KEY, "https://registry.yarnpkg.com");
        self.var(SNM_YARN_REPO_HOST_KEY, "https://repo.yarnpkg.com");
    }

    fn var(&self, key: &str, val: &str) {
        if let Err(_) = env::var(key) {
            env::set_var(key, val);
        }
    }
}
