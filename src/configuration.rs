use std::convert::{TryFrom, TryInto};


#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
    pub port: u16,
    pub host: String,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }

    pub fn connection_string_without_db(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/postgres",
            self.username, self.password, self.host, self.port
        )
    }
}



pub fn get_configuration() -> Result<Settings, config::ConfigError> {

    let base_path = std::env::current_dir()
        .expect("Filed to determine the current directory");
    let configuration_directory = base_path.join("configs");

    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_|"local".into())
        .try_into()
        .expect("Filed to parse APP_ENVIRONMENT");

    let settings = config::Config::builder()
        .add_source(config::File::new("configs/base", config::FileFormat::Yaml))
        .add_source(
            config::File::from(
                    configuration_directory.join(environment.as_str())
        ).required(true))
        .add_source(config::Environment::with_prefix("app").separator("__"));

    match settings.build() {
        Ok(config) => config.try_deserialize(),
        Err(e) => Err(e)
    }
}

pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} in not a support environment. Use either `local` or `production`.",
                other
            ))
        }
    }
}