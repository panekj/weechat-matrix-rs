use crate::{config::ConfigHandle, Servers};
use weechat::hooks::Command;

mod devices;
mod matrix;

use devices::DevicesCommand;
use matrix::MatrixCommand;

pub struct Commands {
    _matrix: Command,
    _devices: Command,
}

impl Commands {
    pub fn hook_all(
        servers: &Servers,
        config: &ConfigHandle,
    ) -> Result<Commands, ()> {
        Ok(Commands {
            _matrix: MatrixCommand::create(servers, config)?,
            _devices: DevicesCommand::create(servers)?,
        })
    }
}
