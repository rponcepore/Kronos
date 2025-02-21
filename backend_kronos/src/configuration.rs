//! configuration.rs

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
    pub application_address: String,
}

#[derive(serde::Deserialize, Clone)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

/*
 *This method allows us to read our config settings from the backend_configuration.yaml file
 *  */
pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Initialize the congif reader
    let settings = config::Config::builder()
    // Add config vals from a file
    .add_source(
        config::File::new("backend_configuration.yaml", config::FileFormat::Yaml)
    )
    .build()?;
    // Try to convert the config values it reads into our Settings type
    settings.try_deserialize::<Settings>()
}


impl DatabaseSettings {
    // This method creates a postgres connection string from settings defined in backend_configuration.yaml
    pub fn connection_string(&self) -> String {
        format! (
            "postgres:://{}:{}@{}:{}/{}",
            self.username,
            self.password,
            self.host,
            self.port,
            self.database_name,
        )
    }
}

mod tests{
    use crate::configuration::get_configuration;

    #[tokio::test]
    async fn test_read_configs () {
        let result = match get_configuration() {
            Ok(result) => true,
            Err(error) => {
                println!("Error: {}", error);
                false
            }
        };
        assert!(result);
    }
}
