use std::convert::{TryFrom, TryInto};
use sqlx::postgres::{PgConnectOptions, PgSslMode};
use sqlx::ConnectOptions;
use serde_aux::field_attributes::deserialize_number_from_string;


#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub database_name: String,

    pub require_ssl: bool,
}

impl DatabaseSettings {
    pub fn without_db(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };

        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(&self.password)
            .port(self.port)
            .ssl_mode(ssl_mode)
    }

    pub fn with_postgres_db(&self) -> PgConnectOptions {
        let mut options = self.without_db().database("postgres");
        options.log_statements(log::LevelFilter::Trace);
        options
    }


    pub fn with_db(&self) -> PgConnectOptions {
        self.without_db().database(&self.database_name)
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