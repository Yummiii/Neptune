use std::{env::args, fs};
use chrono::Utc;
use run_script::ScriptOptions;

pub type GenericError = Box<dyn std::error::Error + Send + Sync>;
fn main() -> Result<(), GenericError> {
    let mut local = Utc::now().format(&args().nth(1).expect("Você não falou o pasta que tenho q salvar o print")).to_string();
    fs::create_dir_all(&local)?;

    let (quantidade, _) = run_script::run_script_or_exit!(r#"find $1../ -type f | wc -l"#, &vec![local.clone()], &ScriptOptions::new());
    let proximo = quantidade.trim().parse::<i32>().unwrap() + 1;
    local.push_str(&format!("{}.png", proximo));

    run_script::run_script_or_exit!(r#"gnome-screenshot -a -c -f $1"#, &vec![local], &ScriptOptions::new());
    Ok(())    
}