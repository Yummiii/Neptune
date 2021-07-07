use crate::GenericError;
use os_pipe::pipe;
use std::{fs::{File}, io::Write, process::Command};

pub fn quantidade_arquivos(pasta: &str) -> Result<i32, GenericError> {
    let (reader, writer) = pipe().unwrap();
    Command::new("/usr/bin/find")
        .args(vec![pasta, "-type", "f"])
        .stdout(writer)
        .spawn()?;

    let wc_out = Command::new("/usr/bin/wc")
        .arg("-l")
        .stdin(reader)
        .output()?;

    Ok(String::from_utf8(wc_out.stdout)?.trim().parse::<i32>()?)
}

pub fn print(local: &str) -> Result<(), GenericError> {
    let out = Command::new("/usr/bin/flameshot")
        .args(vec!["gui", "-r"])
        .output()?;

    if out.stdout != b"screenshot aborted\n" {
        let mut file = File::create(local)?;
        file.write_all(&out.stdout)?;
    }
    Ok(())
}
