use std::collections::HashMap;

use crate::core::ShellOptions;

// pub(crate) struct Shell {
//     name: String,
//     config_path: String,
//     user_config_path: String,
// }

// impl Shell {
//     pub fn new(name: String, config_path: String, user_config_path: String) -> Self {
//         Self {
//             name: name,
//             config_path: config_path,
//             user_config_path: user_config_path,
//         }
//     }

//     /// This function setup shell to use envset
//     /// This function return a error if failed to setup.
//     pub fn init_setup(&self) -> Result<(), String> {
//         Ok(())
//     }

//     /// Returns shell config of the shells
//     /// How to make configs for this.
//     /// ShellOptions as a key, return a new Shell instance
//     /// Every new shell config you have to config it.
//     pub fn from(sop: ShellOptions) -> Self {
//         let mut map = HashMap::new();

//         return map;
//     }
// }

/// Shell is a blueprint for other shell installations
pub trait Shell {
    /// Returns name of the shell
    fn name(&self) -> String;
    /// Returns shell config path
    fn shell_config_path(&self) -> String;
    /// Returns user config path. Mostly in .config/envset/ folder.
    fn user_config_path(&self) -> String;
    /// Setup all the configs and other things to actually work the envset
    fn inits_setup(&self);
    /// Create that shell instance from shell options
    fn new() -> Self;
    /// This function is used to add env
    fn add_env(&self) -> Result<(), String>;
    /// This function is used to delete env
    fn delete_env(&self) -> Result<(), String>;
}

pub struct ZshShell;

impl Shell for ZshShell {
    fn name(&self) -> String {
        todo!()
    }

    fn shell_config_path(&self) -> String {
        todo!()
    }

    fn user_config_path(&self) -> String {
        todo!()
    }

    fn inits_setup(&self) {
        todo!()
    }

    fn new() -> Self {
        todo!()
    }

    fn add_env(&self) -> Result<(), String> {
        todo!()
    }

    fn delete_env(&self) -> Result<(), String> {
        todo!()
    }
}

pub struct FishShell;

impl Shell for FishShell {
    fn name(&self) -> String {
        todo!()
    }

    fn shell_config_path(&self) -> String {
        todo!()
    }

    fn user_config_path(&self) -> String {
        todo!()
    }

    fn inits_setup(&self) {
        todo!()
    }

    fn new() -> Self {
        todo!()
    }

    fn add_env(&self) -> Result<(), String> {
        todo!()
    }

    fn delete_env(&self) -> Result<(), String> {
        todo!()
    }
}

pub struct BashShell;

impl Shell for BashShell {
    fn name(&self) -> String {
        todo!()
    }

    fn shell_config_path(&self) -> String {
        todo!()
    }

    fn user_config_path(&self) -> String {
        todo!()
    }

    fn inits_setup(&self) {
        todo!()
    }

    fn new() -> Self {
        todo!()
    }

    fn add_env(&self) -> Result<(), String> {
        todo!()
    }

    fn delete_env(&self) -> Result<(), String> {
        todo!()
    }
}
