use std::{
    env,
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, BufWriter, SeekFrom, Write},
    path::Path,
};

pub fn get_home_dir() -> String {
    let home = env::var("HOME").expect("Unable to get home dir");
    return home;
}

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
    fn shell_config_path(&self) -> String;
    /// Returns user config path. Mostly in .config/envset/ folder.
    fn user_config_path(&self) -> String;
    /// Setup all the configs and other things to actually work the envset
    fn inits_setup(&self);
    /// Create that shell instance from shell options
    fn new() -> Self;
    /// This function is used to add env
    fn add_env(&self, path: String) -> Result<(), String>;
    /// This function is used to delete env
    fn delete_env(&self, path: String) -> Result<(), String>;

    // Append to a file
    fn write(&self, shell_code: String) -> Result<(), String> {
        match OpenOptions::new()
            .write(true)
            .append(true)
            .open(self.shell_config_path())
        {
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
            Ok(mut file) => delete_line(&mut file, self.user_config_path(), shell_code),
            Err(_) => return Err(String::from("Unable to open shell config file")),
        };
        Ok(())
    }
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

    fn add_env(&self, path: String) -> Result<(), String> {
        todo!()
    }

    fn delete_env(&self, path: String) -> Result<(), String> {
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
        // TODO: create this fish file if the shell is installed.
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
        // write a new file called envset.fish in config folder of fish shell.
        // write the path of actual user config fish file in it.
    }

    fn new() -> Self {
        Self {}
    }

    fn add_env(&self, path: String) -> Result<(), String> {
        // generate the env string using path
        // This will add permanently
        let shell_code = format!("\nfish_add_path {}", path);
        self.write(shell_code); // write that in user config file
        Ok(())
    }

    fn delete_env(&self, path: String) -> Result<(), String> {
        let shell_code = format!("\nfish_add_path {}", path);
        self.delete(shell_code);
        Ok(())
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

    fn add_env(&self, path: String) -> Result<(), String> {
        todo!()
    }

    fn delete_env(&self, path: String) -> Result<(), String> {
        todo!()
    }
}
