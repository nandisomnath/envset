use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{collections::HashMap, fs::OpenOptions};

pub const WORKING_DIR: &str = ".config/envset/";

pub fn get_config_dir() -> String {
    let home_dir = std::env::var("HOME").expect("Unable to get HOME directory");
    let config_dir_path = std::path::Path::new(&home_dir).join(WORKING_DIR);
    config_dir_path.to_str().unwrap().to_string()
}

pub fn init_setup() {
    let home_dir = std::env::var("HOME").expect("Unable to get HOME directory");
    let config_dir_path = Path::new(&home_dir).join(WORKING_DIR);

    if !config_dir_path.exists() {
        std::fs::create_dir_all(&config_dir_path).expect("Unable to create config directory");
    }

    // create bash_profile file if not exists

    let bash_profile = Path::new(&config_dir_path).join("bash_profile");
    File::create(&bash_profile).expect("Unable to create the bash_profile.");
    let bashrc_path = Path::new(&home_dir).join(".bashrc");
    let content = format!("source {}", bash_profile.as_os_str().to_str().unwrap());
    write_to_file(&bashrc_path, content.as_str());

    // create fish_profile file if not exists
    // not need to add the file in tha config.fish
    let fish_path = ".config/fish/conf.d/envset.fish";
    let fishconf_path = Path::new(&home_dir).join(fish_path);
    File::create(&fishconf_path).expect("Unable to create envset.fish file");
}

#[derive(Debug)]
pub enum ShellOptions {
    FISH,
    BASH,
    ZSH,
    UNKOWN(String), // When no matching shell is found
}

/// Indentifies the shell and gives a value from enum Shell.
pub fn get_shell() -> ShellOptions {
    let env_vars: HashMap<String, String> = std::env::vars().collect();

    let shell_path = env_vars
        .get("SHELL")
        .expect("unable to get shell using $SHELL variable")
        .clone();

    if shell_path.contains("fish") {
        return Shell::FISH;
    } else if shell_path.contains("bash") {
        return Shell::BASH;
    } else if shell_path.contains("zsh") {
        return Shell::ZSH;
    }

    return Shell::UNKOWN(shell_path);
}

fn write_to_file(file_path: &std::path::Path, content: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)
        .unwrap();
    writeln!(file, "{}", content).unwrap();
    file.flush().unwrap();
}

pub fn config_bash_env(env_name: String, env_value: String) {
    let bash_profile_path = Path::new(get_config_dir().as_str()).join("bash_profile");
    println!("{:?}", bash_profile_path);
    if env_name.trim() == "PATH" {
        let content = format!("export {}=\"$PATH:{}\"", env_name, env_value);
        write_to_file(&bash_profile_path, content.as_str());
    } else {
        let content = format!("export {}={}", env_name, env_value);
        write_to_file(&bash_profile_path, content.as_str());
    }
}

/// fish uses set to set variables. set -gx variable value
/// unlike bash related shells use export.
/// set program only sets for current shell or child shell otherwise not parmanent
pub fn config_fish_env(env_name: String, env_value: String) {
    let home_dir = std::env::var("HOME").expect("Unable to get HOME directory");
    let fish_path = ".config/fish/conf.d/envset.fish";
    let fishconf_path = Path::new(&home_dir).join(fish_path);

    if env_name.trim() == "PATH" {
        let content = format!("set -gx {} $PATH", env_value);
        write_to_file(&fishconf_path, content.as_str());
    } else {
        let content = format!("set -gx {} {}", env_name, env_value);
        write_to_file(&fishconf_path, content.as_str());
    }
}

pub fn config_zsh_env(env_name: String, env_value: String) {
    let profile_path = Path::new(get_config_dir().as_str()).join("zsh_profile");

    if env_name.trim() == "PATH" {
        let content = format!("export {}=\"$PATH:{}\"", env_name, env_value);
        write_to_file(&profile_path, content.as_str());
    } else {
        let content = format!("export {}={}", env_name, env_value);
        write_to_file(&profile_path, content.as_str());
    }
}

/// Adds env to the current shell. which is determined by $SHELL env variable.
pub fn add_env(env_name: String, env_value: String) {
    let shell = get_shell();
    println!("{:?}", shell);

    match shell {
        Shell::UNKOWN(shell_path) => println!("Unable to add to '{}'", shell_path),
        Shell::BASH => config_bash_env(env_name, env_value),
        Shell::FISH => config_fish_env(env_name, env_value),
        Shell::ZSH => config_zsh_env(env_name, env_value),
    }
}
