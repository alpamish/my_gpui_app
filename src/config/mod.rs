use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database: DatabaseConfig,
    pub app: AppConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub name: String,
}

impl DatabaseConfig {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.name
        )
    }

    pub fn connection_string_without_db(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        )
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub title: String,
    pub width: f32,
    pub height: f32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            database: DatabaseConfig {
                host: "localhost".into(),
                port: 5432,
                username: "postgres".into(),
                password: "postgres".into(),
                name: "myapp".into(),
            },
            app: AppConfig {
                title: "My Application".into(),
                width: 800.0,
                height: 600.0,
            },
        }
    }
}

pub fn load_config() -> Result<Config, config::ConfigError> {
    config::Config::builder()
        .add_source(config::File::with_name("config"))
        .add_source(config::Environment::default())
        .build()?
        .try_deserialize()
}
