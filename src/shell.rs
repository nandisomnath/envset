use std::{
    env,
    path::{Path, PathBuf},
};

pub fn get_home_dir() -> String {
    let home = env::var("HOME").expect("Unable to get home dir");
    return home;
}

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
        String::from("zsh")
    }

    fn shell_config_path(&self) -> String {
        let home = get_home_dir();
        let path = Path::new(&home).join(".zshrc");
        return path
            .to_str()
            .expect("Unable to make path to String")
            .to_string();
    }

    fn user_config_path(&self) -> String {
        let home = get_home_dir();
        let path = Path::new(&home).join(".config/envset/zshrc");
        return path
            .to_str()
            .expect("Unable to make path to String")
            .to_string();
    }

    fn inits_setup(&self) {
        todo!()
    }

    fn new() -> Self {
        Self {}
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
        String::from("fish")
    }

    fn shell_config_path(&self) -> String {
        let home = get_home_dir();
        todo!("create this fish file if the shell is installed.");
        let path = Path::new(&home).join(".config/fish/conf.d/envset.fish");
        return path
            .to_str()
            .expect("Unable to make path to String")
            .to_string();
    }

    fn user_config_path(&self) -> String {
        let home = get_home_dir();
        let path = Path::new(&home).join(".config/envset/envset.fish");
        return path
            .to_str()
            .expect("Unable to make path to String")
            .to_string();
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
