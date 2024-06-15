use anyhow::Result;
use serde::Deserialize;

trait ConfigMerge {
    fn merge(&mut self, to_merge: Self) -> Result<()>;
}

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub server: ServerConfig,
    #[serde(default)]
    pub database: DatabaseConfig,
}

#[derive(Debug, Default, Deserialize)]
pub struct ServerConfig {
    #[serde(default)]
    pub port: u16,
}

#[derive(Debug, Default, Deserialize)]
pub struct DatabaseConfig {
    #[serde(default)]
    pub host: String,
    #[serde(default)]
    pub dbname: String,
    #[serde(default)]
    pub user: String,
    #[serde(default)]
    pub password: String,
}

impl Config {
    pub fn load(env: &str) -> Result<Config> {
        let mut base = Config::load_default()?;
        match env {
            "prod" => {
                let config = std::fs::read_to_string("fixtures/prod.yml")?;
                let to_merge = serde_yaml::from_str(&config)?;
                base.merge(to_merge)?
            }
            "dev" => {
                let config = std::fs::read_to_string("fixtures/dev.yml")?;
                let to_merge = serde_yaml::from_str(&config)?;
                base.merge(to_merge)?
            }
            _ => panic!("Unknown environment"),
        };
        Ok(base)
    }

    fn load_default() -> Result<Config> {
        let config = include_str!("../fixtures/default.yml");
        let config: Config = serde_yaml::from_str(config)?;
        Ok(config)
    }
}

impl ConfigMerge for ServerConfig {
    fn merge(&mut self, to_merge: Self) -> Result<()> {
        if to_merge.port != 0 {
            self.port = to_merge.port;
        }
        Ok(())
    }
}

impl ConfigMerge for DatabaseConfig {
    fn merge(&mut self, to_merge: Self) -> Result<()> {
        if !to_merge.host.is_empty() {
            self.host = to_merge.host;
        }
        if !to_merge.dbname.is_empty() {
            self.dbname = to_merge.dbname;
        }
        if !to_merge.user.is_empty() {
            self.user = to_merge.user;
        }
        if !to_merge.password.is_empty() {
            self.password = to_merge.password;
        }
        Ok(())
    }
}

impl ConfigMerge for Config {
    fn merge(&mut self, to_merge: Self) -> Result<()> {
        self.server.merge(to_merge.server)?;
        self.database.merge(to_merge.database)?;
        Ok(())
    }
}

fn main() -> Result<()> {
    let config = Config::load("prod")?;
    println!("{:?}", config);
    Ok(())
}
