mod cmds;

use chrono::Utc;
use std::fs;
use cmds::quantidade_arquivos;
use crate::cmds::print;

pub type GenericError = Box<dyn std::error::Error + Send + Sync>;

fn main() -> Result<(), GenericError> {
    let mut local = Utc::now().format("/home/yummi/Taiga/%Y-%m/").to_string();
    fs::create_dir_all(&local)?;
    local.push_str(&format!(
        "{}.png",
        quantidade_arquivos(&format!("{}..", &local))? + 1
    ));
    print(&local)?;
    Ok(())
}
