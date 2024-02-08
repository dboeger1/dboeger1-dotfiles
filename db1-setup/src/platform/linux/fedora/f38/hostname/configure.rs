use crate::{
    error::Error,
    subcommand::hostname::subcommand::configure::Args,
};


pub(crate) fn configure(_args: &Args) -> Result<(), Error> {
    println!("<< F38 HOSTNAME CONFIGURE >>");

    Ok(())
}