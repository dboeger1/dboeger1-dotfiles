mod subcommand;


use crate::{
    error::Error,
    platform::Platform,
};
use subcommand::{
    execute_subcommand,
    Subcommand,
};


#[derive(clap::Args, PartialEq, Eq)]
pub struct Args {
    #[command(subcommand)]
    pub(crate) subcommand: Subcommand,
}


pub(crate) fn subcommand_neovim(
    platform: &Platform,
    args: &Args,
) -> Result<(), Error> {
    execute_subcommand(platform, &args.subcommand)
}
