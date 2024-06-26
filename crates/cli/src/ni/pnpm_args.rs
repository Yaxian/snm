use snm_core::model::SnmError;

use super::trait_transform_args::{
    AddCommandArgs, CommandArgsCreatorTrait, DeleteCommandArgs, DlxCommandArgs, ExecCommandArgs,
    InstallCommandArgs, SetCacheArgs,
};

pub struct PnpmArgsTransform;

impl CommandArgsCreatorTrait for PnpmArgsTransform {
    fn get_install_command(&self, args: InstallCommandArgs) -> Result<Vec<String>, SnmError> {
        let mut process_args = vec!["install".to_string()];
        if args.frozen_lockfile {
            process_args.push("--frozen-lockfile".to_string());
        }

        Ok(process_args)
    }

    fn get_add_command(&self, args: AddCommandArgs) -> Result<Vec<String>, SnmError> {
        let mut process_args = vec!["add".to_string(), args.package_spec];
        if args.save_prod {
            process_args.push("--save".to_string());
        } else if args.save_dev {
            process_args.push("--save-dev".to_string());
        } else if args.save_optional {
            process_args.push("--save-optional".to_string());
        } else if args.save_exact {
            process_args.push("--save-exact".to_string());
        } else if args.save_peer {
            process_args.push("--save-peer".to_string());
        } else if args.global {
            process_args.push("--global".to_string());
        }
        Ok(process_args)
    }

    fn get_delete_command(&self, args: DeleteCommandArgs) -> Result<Vec<String>, SnmError> {
        let process_args = vec!["remove".to_string(), args.package_spec];
        Ok(process_args)
    }

    fn get_dlx_command(&self, args: DlxCommandArgs) -> Result<Vec<String>, SnmError> {
        let mut process_args = vec!["dlx".to_string()];
        process_args.append(&mut args.package_spec.clone());
        Ok(process_args)
    }

    fn get_exec_command(&self, args: ExecCommandArgs) -> Result<Vec<String>, SnmError> {
        let mut process_args: Vec<String> = vec!["exec".to_string()];
        process_args.append(&mut args.package_spec.clone());
        Ok(process_args)
    }

    fn get_run_command(
        &self,
        args: super::trait_transform_args::RunCommandArgs,
    ) -> Result<Vec<String>, SnmError> {
        let mut process_args: Vec<String> = vec!["run".to_string()];
        process_args.append(&mut args.args.clone());
        Ok(process_args)
    }

    fn get_set_cache_command(&self, args: SetCacheArgs) -> Result<Vec<String>, SnmError> {
        let process_args = vec!["set".to_string(), "store-dir".to_string(), args.cache_path];
        Ok(process_args)
    }
}
