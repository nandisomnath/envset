use std::env;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{collections::HashMap, fs::OpenOptions};


pub fn envset_dir() -> PathBuf {
    let mut home_dir = env::home_dir().unwrap();
    home_dir.push(".config/envset");
    return home_dir;
}

#[derive(Debug)]
pub enum ShellOptions {
    FISH,
    BASH,
    ZSH,
    UNKOWN(String), // When no matching shell is found
}

/// Indentifies the shell and gives a value from enum Shell.
pub fn get_shell(shell: &str) -> ShellOptions {
    match shell {
        "fish" => return ShellOptions::ZSH,
        "bash" => return ShellOptions::BASH,
        "zsh" => return ShellOptions::ZSH,
        _ => return ShellOptions::UNKOWN(shell.to_string())
    }
}

// fn write_to_file(file_path: &std::path::Path, content: &str) {
//     let mut file = OpenOptions::new()
//         .append(true)
//         .create(true)
//         .open(file_path)
//         .unwrap();
//     writeln!(file, "{}", content).unwrap();
//     file.flush().unwrap();
// }

// // In bash to add path we have to use export.
// // basically it is old model and only use to add path for current shell.
// // TODO: implement bash path setter using export and adding it in bash_profile to make it permanent.
// pub fn config_bash_env(env_value: String) {
//     let bash_profile_path = Path::new(get_config_dir().as_str()).join("bash_profile");
//     println!("{:?}", bash_profile_path);
//     if env_name.trim() == "PATH" {
//         let content = format!("export {}=\"$PATH:{}\"", env_name, env_value);
//         write_to_file(&bash_profile_path, content.as_str());
//     } else {
//         let content = format!("export {}={}", env_name, env_value);
//         write_to_file(&bash_profile_path, content.as_str());
//     }
// }

// // In fish adding permanent path variable is easy just use builtin functions
// // fish_add_path  $HOME/.config/emacs/bin/
// // This function will add the path permanently.
// pub fn config_fish_env(env_value: String) {
//     let home_dir = std::env::var("HOME").expect("Unable to get HOME directory");
//     let fish_path = ".config/fish/conf.d/envset.fish";
//     let fishconf_path = Path::new(&home_dir).join(fish_path);

//     if env_name.trim() == "PATH" {
//         let content = format!("set -gx {} $PATH", env_value);
//         write_to_file(&fishconf_path, content.as_str());
//     } else {
//         let content = format!("set -gx {} {}", env_name, env_value);
//         write_to_file(&fishconf_path, content.as_str());
//     }
// }

// /// Same as bash
// /// Some extra thing I found that it have a path array which
// /// is used to sync with other paths.
// pub fn config_zsh_env(env_value: String) {
//     let profile_path = Path::new(get_config_dir().as_str()).join("zsh_profile");

//     if env_name.trim() == "PATH" {
//         let content = format!("export {}=\"$PATH:{}\"", env_name, env_value);
//         write_to_file(&profile_path, content.as_str());
//     } else {
//         let content = format!("export {}={}", env_name, env_value);
//         write_to_file(&profile_path, content.as_str());
//     }
// }

/// Adds env to the current shell. which is determined by $SHELL env variable.
pub fn add_env(env_value: String, remove_path: bool, init_setup: bool, shell: &str) {
    let shop = get_shell(shell);
    println!("{:?}", shop);

    match shop {
        ShellOptions::UNKOWN(shell_path) => println!("Unable to add to '{}'", shell_path),
        ShellOptions::BASH => {
            // TODO: configure the bash use Shell trait fucntions
        }
        ShellOptions::FISH => { //TODO: configure this shells usng Shell trait functions
        }
        ShellOptions::ZSH => { //TODO: configure this shells usng Shell trait functions
        }
    }
}
