use clap::{command, CommandFactory, Parser};
use colored::*;
use dialoguer::{theme::ColorfulTheme, Select};
use manage_command::ManageCommands;
use ni::{
    npm_args::NpmArgsTransform,
    pnpm_args::PnpmArgsTransform,
    trait_transform_args::{CommandArgsCreatorTrait, InstallCommandArgs},
    yarn_args::YarnArgsTransform,
    yarnpkg_args::YarnPkgArgsTransform,
};
use semver::{Prerelease, Version};
use snm_command::SnmCommands;
use snm_core::{
    config::SnmConfig,
    model::{
        dispatch_manage::DispatchManage, package_json::PackageManager, snm_error::handle_snm_error,
        trait_manage::ManageTrait, PackageJson, SnmError,
    },
    println_success,
};
use snm_node::snm_node::SnmNode;
use snm_npm::snm_npm::SnmNpm;
use snm_pnpm::snm_pnpm::SnmPnpm;
use snm_yarn::{snm_yarn::SnmYarn, snm_yarnpkg::SnmYarnPkg};
use std::{
    fs,
    ops::Not,
    process::{Command, Stdio},
};
mod manage_command;
mod ni;
mod snm_command;

#[derive(Parser, Debug)]
struct SnmCli {
    #[command(subcommand)]
    command: SnmCommands,
}

#[tokio::main]
async fn main() -> Result<(), SnmError> {
    SnmConfig::new().init()?;

    if let Err(error) = execute_cli().await {
        handle_snm_error(error);
    }

    Ok(())
}

async fn execute_cli() -> Result<(), SnmError> {
    let cli = SnmCli::parse();

    match cli.command {
        // manage start
        SnmCommands::Yarn { command } => {
            exec_manage(command, Box::new(SnmYarn::new())).await?;
        }
        SnmCommands::Pnpm { command } => {
            exec_manage(command, Box::new(SnmPnpm::new())).await?;
        }
        SnmCommands::Npm { command } => {
            exec_manage(command, Box::new(SnmNpm::new())).await?;
        }
        SnmCommands::Node { command } => {
            exec_manage(command, Box::new(SnmNode::new())).await?;
        }
        // manage end

        // snm command start
        SnmCommands::Install(args) => {
            execute_command(|creator| creator.get_install_command(args)).await?;
        }
        SnmCommands::CI(_) => {
            execute_command(|creator| {
                creator.get_install_command(InstallCommandArgs {
                    frozen_lockfile: true,
                })
            })
            .await?;
        }
        SnmCommands::Add(args) => {
            execute_command(|creator| creator.get_add_command(args)).await?;
        }
        SnmCommands::Delete(args) => {
            execute_command(|creator| creator.get_delete_command(args)).await?;
        }
        SnmCommands::Query => todo!(),
        // snm command end
        SnmCommands::FigSpec => {
            let mut output = Vec::new();
            clap_complete::generate(
                clap_complete_fig::Fig,
                &mut SnmCli::command(),
                "snm",
                &mut output,
            );
            let output_string = String::from_utf8(output).unwrap();

            if let Some(home) = dirs::home_dir() {
                let dir = home.join(".fig").join("autocomplete").join("build");

                if dir.exists().not() {
                    fs::create_dir_all(&dir)?;
                }

                let spec_path_buf = dir.join("snm.ts");

                if spec_path_buf.exists() {
                    fs::remove_file(&spec_path_buf)?;
                }

                fs::write(&spec_path_buf, &output_string)?;

                println_success!(
                    "Fig spec file has been created at {}",
                    spec_path_buf.display()
                );
            }
        }
        SnmCommands::Bump => {
            let package_json = PackageJson::from_dir_path(None)?;

            let v: Version =
                Version::parse(package_json.version.unwrap_or("0.0.0".to_string()).as_str())?;

            let mut inc_prerelease = v.clone();

            let x = v.pre.parse::<u8>().unwrap();

            inc_prerelease.pre = Prerelease::new((x + 1).to_string().as_str())?;

            let major = Version::new(v.major + 1, v.minor, v.patch);
            let minor = Version::new(v.major, v.minor + 1, v.patch);
            let patch = Version::new(v.major, v.minor, v.patch + 1);

            let prerelease = Prerelease::new("0")?;

            let mut major_prerelease = Version::new(v.major + 1, 0, 0);
            major_prerelease.pre = prerelease.clone();

            let mut minor_prerelease = Version::new(v.major, v.minor + 1, 0);
            minor_prerelease.pre = prerelease.clone();

            let mut patch_prerelease = Version::new(v.major, v.minor, v.patch + 1);
            patch_prerelease.pre = prerelease.clone();

            let selections = &[
                format!("{:<10} {}", "major", major.to_string().bright_black()),
                format!("{:<10} {}", "minor", minor.to_string().bright_black()),
                format!("{:<10} {}", "patch", patch.to_string().bright_black()),
                format!(
                    "{:<10} {}",
                    "premajor",
                    major_prerelease.to_string().bright_black()
                ),
                format!(
                    "{:<10} {}",
                    "preminor",
                    minor_prerelease.to_string().bright_black()
                ),
                format!(
                    "{:<10} {}",
                    "prepatch",
                    patch_prerelease.to_string().bright_black()
                ),
                format!(
                    "{:<10} {}",
                    "prerelease",
                    inc_prerelease.to_string().bright_black()
                ),
            ];
            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt(format!(
                    "请选择要升级的版本号: {} ",
                    v.to_string().bright_purple()
                ))
                .default(0) // 默认选中第一个选项
                .items(&selections[..])
                .interact()
                .unwrap();

            println!("您选择了: {} , {:?}", selections[selection], v);
        }
    }
    Ok(())
}

pub async fn exec_manage(
    command: ManageCommands,
    manager: Box<dyn ManageTrait>,
) -> Result<(), SnmError> {
    let dispatcher = DispatchManage::new(manager);

    let trim_version = |version: String| version.trim_start_matches(['v', 'V']).trim().to_owned();

    match command {
        ManageCommands::Default { version } => {
            dispatcher.set_default(&trim_version(version)).await?
        }
        ManageCommands::Install { version } => {
            dispatcher.install(&trim_version(version)).await?;
        }
        ManageCommands::Uninstall { version } => {
            dispatcher.un_install(&trim_version(version)).await?;
        }
        ManageCommands::List => {
            dispatcher.list().await?;
        }
        ManageCommands::ListRemote { all } => {
            dispatcher.list_remote(all).await?;
        }
    };
    Ok(())
}

async fn execute_command<F>(get_command_args: F) -> Result<(), SnmError>
where
    F: FnOnce(&dyn CommandArgsCreatorTrait) -> Result<Vec<String>, SnmError>,
{
    let package_manager = get_package_manager()?;

    let manager = get_manage(&package_manager).await?;

    let command_args_creator = get_command_args_creator(&package_manager)?;

    let args = get_command_args(command_args_creator.as_ref())?;

    let dispatcher = DispatchManage::new(manager);
    let (_, bin_path_buf) = dispatcher.ensure_strict_package_manager().await?;

    println_success!(
        "Use {}. {}",
        format!("{:<8}", package_manager.version).bright_green(),
        format!("by {}", bin_path_buf.display()).bright_black()
    );

    Command::new(bin_path_buf.display().to_string())
        .args(&args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .stdin(Stdio::inherit())
        .spawn()
        .and_then(|process| process.wait_with_output())?;
    Ok(())
}

fn get_package_manager() -> Result<PackageManager, SnmError> {
    let package_json = PackageJson::from_dir_path(None)?;
    package_json.parse_package_manager()
}

fn get_command_args_creator(
    package_manager: &PackageManager,
) -> Result<Box<dyn CommandArgsCreatorTrait>, SnmError> {
    let x: Box<dyn CommandArgsCreatorTrait> = match package_manager.name.as_str() {
        "npm" => Box::new(NpmArgsTransform {}),
        "pnpm" => Box::new(PnpmArgsTransform {}),
        "yarn" => {
            if get_is_less_2(&package_manager.version)? {
                Box::new(YarnArgsTransform {})
            } else {
                Box::new(YarnPkgArgsTransform {})
            }
        }
        _ => panic!("Unsupported package manager"),
    };
    Ok(x)
}

async fn get_manage(package_manager: &PackageManager) -> Result<Box<dyn ManageTrait>, SnmError> {
    let manager: Box<dyn ManageTrait> = match package_manager.name.as_str() {
        "npm" => {
            let manager = SnmNpm::new();
            Box::new(manager)
        }
        "pnpm" => Box::new(SnmPnpm::new()),
        "yarn" => {
            if get_is_less_2(&package_manager.version)? {
                Box::new(SnmYarn::new())
            } else {
                Box::new(SnmYarnPkg::new())
            }
        }
        _ => {
            return Err(SnmError::UnsupportedPackageManager {
                name: package_manager.name.to_string(),
                version: package_manager.version.to_string(),
            })
        }
    };
    Ok(manager)
}

fn get_is_less_2(v: &str) -> Result<bool, SnmError> {
    let ver = Version::parse(v)?;
    let is_less_2 = ver < Version::parse("2.0.0")?;
    Ok(is_less_2)
}
