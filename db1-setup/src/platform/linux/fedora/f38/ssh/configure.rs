use crate::{
    error::Error,
    subcommand::ssh::subcommand::configure::Args,
};


pub(crate) fn configure(_args: &Args) -> Result<(), Error> {
    println!("<< F38 SSH CONFIGURE >>");

    Ok(())
}