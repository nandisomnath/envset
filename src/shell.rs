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
    pub fn init_setup(&self) -> Result<_, String> {}

    /// Returns shell config of the shells
    pub fn configs() -> HashMap<ShellOptions, Self> {}
}
