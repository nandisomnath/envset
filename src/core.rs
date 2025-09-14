use std::collections::HashMap;


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


pub fn config_bash_env() { todo!()}
pub fn config_zsh_env() { todo!()}
pub fn config_fish_env() { todo!()}


/// Adds env to the current shell. which is determined by $SHELL env variable.
pub fn add_env(env_name: String, env_value: String) {
    let shell = get_shell();


    match shell {
        Shell::UNKOWN(shell_path) => println!("Unable to add to '{}'", shell_path),
        Shell::BASH => config_bash_env(),
        Shell::FISH => config_fish_env(),
        Shell::ZSH => config_zsh_env(),
    }

}
