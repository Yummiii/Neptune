use std::io::{self, Read};
use chrono::{Datelike, Utc};
use tokio::fs;
use glob::glob;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = Vec::new();
    let mut stdin = io::stdin(); 
    stdin.read_to_end(&mut buf)?;

    let now = Utc::now();
    let local = format!("/home/yummi/Taiga/Printis/{}-{:02}", now.year(), now.month());
    fs::create_dir_all(&local).await?;
    let quantidade = glob(&format!("{}/../**/*.*", &local))?.count() + 1;
    fs::write(format!("{}/{}.png", local, quantidade), buf).await?;

    Ok(())
}