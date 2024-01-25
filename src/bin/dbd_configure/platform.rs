#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub(crate) use linux::*;


use crate::{
    error::Error,
    source_destination::SourceDestination,
};

pub(crate) struct Platform {
    pub(crate) neovim_paths: Option<SourceDestination>,
    pub(crate) tmux_paths: Option<SourceDestination>,
    pub(crate) install_packages: Option<fn() -> Result<(), Error>>,
}
