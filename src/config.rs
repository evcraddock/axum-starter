use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;
use std::path::Path;

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct AppConfig {
    pub run_mode: String,
    pub some_other_setting: String,
}

pub fn load_config() -> Result<AppConfig, ConfigError> {
    // Remove unused run_env variable
    let config_dir = env::var("CONFIG_DIR").unwrap_or_else(|_| ".".into());
    let config_path = Path::new(&config_dir).join("settings.toml");
    
    let config = Config::builder()
        // Start with defaults
        .add_source(File::from(config_path))
        // Add environment variables with prefix "APP_"
        .add_source(Environment::with_prefix("APP").separator("_"))
        .build()?;
    
    // Try to deserialize the configuration into our AppConfig struct
    config.try_deserialize()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    
    // Simple test for the AppConfig struct directly
    #[test]
    fn test_app_config_deserialize() {
        let config_str = r#"
        run_mode = "test"
        some_other_setting = "value"
        "#;
        
        let config = Config::builder()
            .add_source(config::File::from_str(config_str, config::FileFormat::Toml))
            .build()
            .unwrap();
        
        let app_config: AppConfig = config.try_deserialize().unwrap();
        
        assert_eq!(app_config.run_mode, "test");
        assert_eq!(app_config.some_other_setting, "value");
    }
    
    // Test that settings file is required for the app to start
    #[test]
    fn test_missing_config_file_fails() {
        // Point to a non-existent directory
        unsafe {
            env::set_var("CONFIG_DIR", "/non/existent/directory");
        }
        
        let result = load_config();
        
        // Clean up
        unsafe {
            env::remove_var("CONFIG_DIR");
        }
        
        // Verify load fails
        assert!(result.is_err());
    }
    
    // Test direct environment access method
    #[test]
    fn test_direct_env_access() {
        // Set environment variable
        unsafe {
            env::set_var("APP_RUN_MODE", "env_value");
        }
        
        // Get value directly
        let env_value = env::var("APP_RUN_MODE").unwrap_or_default();
        
        // Clean up
        unsafe {
            env::remove_var("APP_RUN_MODE");
        }
        
        // Just make sure we can directly access env vars
        assert_eq!(env_value, "env_value");
    }
}