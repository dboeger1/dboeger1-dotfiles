use crate::{
    error::Error,
    ssh::configure::Args,
};


pub(crate) fn configure(_args: &Args) -> Result<(), Error> {
    println!("<< F38 SSH CONFIGURE >>");

    Ok(())
}
