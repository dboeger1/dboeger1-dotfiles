use crate::{
    error::Error,
    platform::Platform,
};


pub(crate) fn subcommand_verify(_platform: &Platform) -> Result<(), Error> {
    println!("=== SUBCOMMAND: TMUX VERIFY ===");

    Ok(())
}
