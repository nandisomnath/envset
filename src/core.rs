use std::collections::HashMap;


#[derive(Debug)]
pub enum Shell {
    FISH,
    BASH,
    ZSH,
    UNKOWN
}


/// Indentifies the shell and gives a value of enum Shell.
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


    return Shell::UNKOWN;

}
