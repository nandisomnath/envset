use std::collections::HashMap;

#[derive(Debug)]
pub enum Shell {
    FISH,
    BASH,
    ZSH,
    UNKOWN(String), // When no matching shell is found
}

impl Shell {
    /// Indentifies the shell and gives a value from enum Shell.
    pub fn get_shell() -> Shell {
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

        // This shell path is returned for logging.
        return Shell::UNKOWN(shell_path);
    }
}

pub struct ShellConfig {
    /// Config for envset application
    pub app_conf_path: Option<String>,
    /// Config for current shell
    pub shell_conf_path: Option<String>,
}

impl ShellConfig {
    pub fn new(app_conf_path: Option<&str>, shell_conf_path: Option<&str>) -> ShellConfig {
        let acp = app_conf_path.unwrap_or("");
        let scp = shell_conf_path.unwrap_or("");

        ShellConfig {
            app_conf_path: Some(acp.to_string()),
            shell_conf_path: Some(scp.to_string()),
        }
    }

    pub fn unkown() -> ShellConfig {
        ShellConfig::new(None, None)
    }

    // Defaults
    // Path are all relative to $HOME
    // Update this file to add new shell support
    /// shell value need to get config of current shell use get_shell function
    pub fn get_configs(shell: Shell) -> ShellConfig {
        match shell {
            Shell::ZSH => ShellConfig::new(Some(".config/envset/zshrc"), Some(".zshrc")),
            Shell::BASH => ShellConfig::new(Some(".config/envset/bashrc"), Some(".bashrc")),
            Shell::FISH => ShellConfig::new(
                Some(".config/fish/config.fish"),
                Some(".config/fish/conf.d/envset.fish"),
            ),
            Shell::UNKOWN(_) => ShellConfig::unkown(),
        }
    }
}
