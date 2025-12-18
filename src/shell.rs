use std::collections::HashMap;

use crate::core::ShellOptions;

pub(crate) struct Shell {
    name: String,
    config_path: String,
    user_config_path: String,
}

impl Shell {
    pub fn new(name: String, config_path: String, user_config_path: String) -> Self {
        Self {
            name: name,
            config_path: config_path,
            user_config_path: user_config_path,
        }
    }

    /// This function setup shell to use envset
    /// This function return a error if failed to setup.
    pub fn init_setup(&self) -> Result<(), String> {
        Ok(())
    }

    /// Returns shell config of the shells
    /// How to make configs for this.
    /// ShellOptions as a key, return a new Shell instance
    /// Every new shell config you have to config it.
    pub fn from(sop: ShellOptions) -> Self {
        let mut map = HashMap::new();

        return map;
    }
}
