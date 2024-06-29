use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub night: String,
    pub morning: String,
    pub evening: String,
}