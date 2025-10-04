use std::{collections::HashMap, fs::OpenOptions};
use std::io::Write;


pub const WORKING_DIR: &str = ".config/envset/";

pub fn get_config_dir() -> String {
    let home_dir = std::env::var("HOME").expect("Unable to get HOME directory");
    let config_dir_path = std::path::Path::new(&home_dir).join(WORKING_DIR);
    config_dir_path.to_str().unwrap().to_string()
}

pub fn init_setup() {
    let home_dir = std::env::var("HOME").expect("Unable to get HOME directory");
    let config_dir_path = std::path::Path::new(&home_dir).join(WORKING_DIR);

    if !config_dir_path.exists() {
        std::fs::create_dir_all(&config_dir_path).expect("Unable to create config directory");
    }

    // create bash_profile file if not exists
    // create fish_profile file if not exists
}


#[derive(Debug)]
pub enum Shell {
    FISH,
    BASH,
    ZSH,
    UNKOWN(String) // When no matching shell is found
}


/// Indentifies the shell and gives a value from enum Shell.
pub fn get_shell() -> Shell {

    let env_vars: HashMap<String, String> = std::env::vars().collect();

    let shell_path = env_vars.get("SHELL")
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
        .open(file_path).unwrap();
    writeln!(file, "{}", content).unwrap();
    file.flush().unwrap();

}

pub fn config_bash_env(env_name: String, env_value: String) { 
    let bash_profile_path = std::path::Path::new(get_config_dir().as_str()).join("bash_profile");

    if env_name.trim() == "PATH" {
        let content = format!("export {}=\"$PATH:{}\"", env_name, env_value);
        write_to_file(&bash_profile_path, content.as_str());
    } else {
        let content = format!("export {}={}", env_name, env_value);
        write_to_file(&bash_profile_path, content.as_str());
    }
}
pub fn config_zsh_env(env_name: String, env_value: String) { todo!()}
pub fn config_fish_env(env_name: String, env_value: String) { todo!()}


/// Adds env to the current shell. which is determined by $SHELL env variable.
pub fn add_env(env_name: String, env_value: String) {
    let shell = get_shell();


    match shell {
        Shell::UNKOWN(shell_path) => println!("Unable to add to '{}'", shell_path),
        Shell::BASH => config_bash_env(env_name, env_value),
        Shell::FISH => config_fish_env(env_name, env_value),
        Shell::ZSH => config_zsh_env(env_name, env_value),
    }

}
