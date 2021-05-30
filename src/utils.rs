use std::{io::Write, process::{Command, Stdio}};
use crate::GenericError;

pub fn send(txt: String) -> Result<(), GenericError> {
    let cmd = Command::new("/usr/bin/xclip")
        .args(vec!["-selection", "clipboard"])
        .stdin(Stdio::piped())
        .spawn()?;
    cmd.stdin.unwrap().write_all(txt.as_bytes())?;
    Ok(())
}