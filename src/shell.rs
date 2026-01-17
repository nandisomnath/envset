use std::{
    env::{self, temp_dir},
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, BufWriter, Write},
    path::PathBuf,
};

use clap::error::Result;

/// shell_code - is the line to be deleted.
fn delete_line(user_config_path: PathBuf, shell_code: String) -> Result<(), String> {
    let mut tmp_file = temp_dir();
    tmp_file.push("envset.zsh");
    fs::copy(user_config_path, tmp_file);

    Ok(())
}

/// Shell is a blueprint for other shell installations
pub trait Shell {
    /// Returns name of the shell
    fn name(&self) -> String;
    /// Returns shell config path
    fn shell_config_path(&self) -> PathBuf;
    /// Returns user config path. Mostly in .config/envset/ folder.
    fn user_config_path(&self) -> PathBuf;
    /// Setup all the configs and other things to actually work the envset
    fn inits_setup(&self);
    /// Create that shell instance from shell options
    fn new() -> Self;
    /// This function is used to add env
    fn add_env(&self, env_string: String) -> Result<(), String>;
    /// This function is used to delete env
    fn delete_env(&self, env_string: String) -> Result<(), String>;
    /// Returns the shell code
    fn get_shell_code(&self, env_value: String) -> String;

    // Append to a file
    fn write(&self, shell_code: String) -> Result<(), String> {
        let sconfpath = self.shell_config_path();
        match OpenOptions::new().write(true).append(true).open(sconfpath) {
            Ok(mut file) => file.write_all(shell_code.as_bytes()),
            Err(_) => return Err(String::from("Unable to open shell config file")),
        };
        Ok(())
    }

    // Remove from a file
    fn delete(&self, shell_code: String) -> Result<(), String> {
        let user_config_path = self.user_config_path();
        delete_line(user_config_path, shell_code)?;
        Ok(())
    }
}

pub struct ZshShell;

impl Shell for ZshShell {
    fn name(&self) -> String {
        String::from("zsh")
    }

    fn shell_config_path(&self) -> PathBuf {
        let home = get_home_dir();
        let mut path = PathBuf::from(&home);
        path.push(".zshrc");
        return path;
    }

    fn user_config_path(&self) -> PathBuf {
        let home = get_home_dir();
        let mut path = PathBuf::from(&home);
        path.push(".config/envset/zshrc");
        return path;
    }

    fn inits_setup(&self) {
        todo!()
    }

    fn new() -> Self {
        Self {}
    }

    fn add_env(&self, path: String) -> Result<(), String> {
        let shell_code = format!("");
        self.write(shell_code);
        Ok(())
    }

    fn delete_env(&self, path: String) -> Result<(), String> {
        todo!()
    }

    fn get_shell_code(&self, env_value: String) -> String {
        todo!()
    }
}

pub struct FishShell;

impl Shell for FishShell {
    fn name(&self) -> String {
        String::from("fish")
    }

    fn shell_config_path(&self) -> PathBuf {
        let home = env::var("HOME").unwrap();
        // TODO: create this fish file if the shell is installed.
        let mut path = PathBuf::from(&home);
        path.push(".config/fish/conf.d/envset.fish");
        return path;
    }

    fn user_config_path(&self) -> PathBuf {
        let home = env::var("HOME").unwrap();
        let mut path = PathBuf::from(&home);
        path.push(".config/envset/envset.fish");
        return path;
    }

    fn inits_setup(&self) {
        // write a new file called envset.fish in config folder of fish shell.
        // let mut user_conf_path = Path::new(self.user_config_path());
        // write the path of actual user config fish file in it.
    }

    fn new() -> Self {
        Self {}
    }

    fn get_shell_code(&self, env_value: String) -> String {
        return format!("set -gx {} $PATH", env_value);
    }

    fn add_env(&self, path: String) -> Result<(), String> {
        // generate the env string using path
        // This will add permanently
        let shell_code = self.get_shell_code(path);
        self.write(shell_code); // write that in user config file
        Ok(())
    }

    fn delete_env(&self, path: String) -> Result<(), String> {
        let shell_code = self.get_shell_code(path);
        self.delete(shell_code);
        Ok(())
    }
}

// pub struct BashShell;

// impl Shell for BashShell {
//     fn name(&self) -> String {
//         todo!()
//     }

//     fn shell_config_path(&self) -> PathBuf {
//         todo!()
//     }

//     fn user_config_path(&self) -> PathBuf {
//         todo!()
//     }

//     fn inits_setup(&self) {
//         todo!()
//     }

//     fn new() -> Self {
//         todo!()
//     }

//     fn add_env(&self, path: String) -> Result<(), String> {
//         todo!()
//     }

//     fn delete_env(&self, path: String) -> Result<(), String> {
//         todo!()
//     }
// }
