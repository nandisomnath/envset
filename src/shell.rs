use std::{
    env,
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, BufWriter, Write},
    path::{Path, PathBuf},
};

/// shell_code - is the line to be deleted.
fn delete_line(file: &mut File, user_confg_path: String, shell_code: String) {
    let reader = BufReader::new(file);
    let mut file_path = env::temp_dir();
    file_path.push("envset.fish");
    let outfile = OpenOptions::new()
        .write(true)
        .open(file_path.as_path())
        .unwrap();
    let mut writer = BufWriter::new(outfile);

    let mut current_line;
    for line in reader.lines() {
        current_line = line.expect("Unable to get the line");

        if current_line == shell_code {
            continue;
        }

        writer.write_all(current_line.as_bytes()).unwrap();
    }
    fs::copy(file_path.clone(), user_confg_path);
    fs::remove_file(file_path);
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
        match OpenOptions::new()
            .write(true)
            .append(true)
            .open(self.shell_config_path())
        {
            Ok(mut file) => delete_line(
                &mut file,
                self.user_config_path().to_str().unwrap().to_string(),
                shell_code,
            ),
            Err(_) => return Err(String::from("Unable to open shell config file")),
        };
        Ok(())
    }
}

// pub struct ZshShell;
//
// impl Shell for ZshShell {
//     fn name(&self) -> String {
//         String::from("zsh")
//     }
//
//     fn shell_config_path(&self) -> PathBuf {
//         let home = get_home_dir();
//         let mut path = PathBuf::from(&home);
//         path.push(".zshrc");
//         return path;
//     }
//
//     fn user_config_path(&self) -> PathBuf {
//         let home = get_home_dir();
//         let mut path = PathBuf::from(&home);
//         path.push(".config/envset/zshrc");
//         return path;
//     }
//
//     fn inits_setup(&self) {
//         todo!()
//     }
//
//     fn new() -> Self {
//         Self {}
//     }
//
//     fn add_env(&self, path: String) -> Result<(), String> {
//         let shell_code = format!("");
//         self.write(shell_code);
//         Ok(())
//     }
//
//     fn delete_env(&self, path: String) -> Result<(), String> {
//         todo!()
//     }
// }
//
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
