mod fedora;


use crate::{
    error::Error,
    platform::Platform,
};
use dboeger1_dotfiles::{
    CARGO_NAME,
    OS_INFO,
};
use lazy_static::lazy_static;
use os_info::Type;
use std::{
    path::PathBuf,
    process::Command,
    str::from_utf8,
};



lazy_static! {
    pub(crate) static ref PLATFORM: Option<&'static Platform> =
        match OS_INFO.os_type() {
            Type::Fedora => *fedora::PLATFORM,
            _ => None,
        };

    // User paths.
    pub(crate) static ref INSTALL_DIR: PathBuf =
        PathBuf::from(format!("/opt/{}", CARGO_NAME));
}


pub(crate) fn dnf_install<I>(packages: I) -> Result<(), Error>
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let mut dnf_command = Command::new("dnf");
    dnf_command.arg("install");
    packages
        .into_iter()
        .for_each(|package| {
            dnf_command.arg(package.as_ref());
        });

    let mut dnf_string = dnf_command
        .get_program()
        .to_os_string();
    dnf_command
        .get_args()
        .for_each(|arg| dnf_string.push(format!(
            " {}",
            arg.to_string_lossy(),
        )));

    let dnf_output = dnf_command
        .output()
        .map_err(|error| Error {
            message: format!(
                "failed to execute dnf command \"{}\"",
                dnf_string.to_string_lossy(),
            ),
            source: Some(error),
        })?;
    if !dnf_output.status.success() {
        // command
        eprintln!("==== command ====");
        eprintln!("{}", dnf_string.to_string_lossy());

        // exit code
        let dnf_exit_code = dnf_output
            .status
            .code();
        eprintln!("==== exit code ====");
        eprintln!(
            "{}",
            dnf_exit_code.map_or_else(
                || "<failed to retrieve>".to_string(),
                |status| status.to_string()
            )
        );

        // stdout
        let dnf_stdout = from_utf8(&dnf_output.stdout);
        eprintln!("==== stdout ====");
        if dnf_stdout.is_ok() {
            eprintln!("{}", dnf_stdout.unwrap_or("<failed to retrieve>"));
        }

        // stderr
        let dnf_stderr = from_utf8(&dnf_output.stderr);
        eprintln!("==== stderr ====");
        if dnf_stderr.is_ok() {
            eprintln!("{}", dnf_stderr.unwrap_or("<failed to retrieve>"));
        }

        return Err(Error {
            message: format!(
                "dnf command \"{}\" failed",
                dnf_string.to_string_lossy(),
            ),
            source: None,
        });
    }

    Ok(())
}
