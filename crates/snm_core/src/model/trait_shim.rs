use std::path::PathBuf;

use super::trait_shared_behavior::SharedBehaviorTrait;

pub trait ShimTrait: SharedBehaviorTrait {
    fn get_strict_shim_binary_path_buf(&self, bin_name: &str, version: &str) -> PathBuf;

    fn get_strict_shim_version(&self) -> String;

    fn download_condition(&self, version: &str) -> bool;

    fn get_runtime_binary_file_path_buf(&self, bin_name: &str, version: &str) -> PathBuf;

    fn check_satisfy_strict_mode(&self, bin_name: &str);

    fn check_default_version(&self, tuple: &(Vec<String>, Option<String>)) -> String;
}
