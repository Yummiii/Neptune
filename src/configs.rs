use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub nome: String,
    pub ip: String,
    pub porta: u32,
    pub senha: Option<String>,
    pub priv_key: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Configs {
    pub tamanho_senha: u32,
    pub profiles: Option<Vec<Profile>>
}

impl Default for Configs {
    fn default() -> Self {
        Self {
            tamanho_senha: 50,
            profiles: None
        }
    }
}