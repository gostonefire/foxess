extern crate alloc;
use std::{env, fs};
use std::path::PathBuf;
use thiserror::Error;
use foxess::Fox;
use foxess::fox_variables::BatTemperature;

#[cfg(feature = "async")]
#[tokio::test]
async fn it_works() {
    let api_key = read_credential("fox_ess_api_key").unwrap_or_else(|e| {
        panic!("fox_ess_api_key not found in credstore: {e}");
    });
    let sn = read_credential("fox_ess_inverter_sn").unwrap_or_else(|e| {
        panic!("fox_ess_inverter_sn not found in credstore: {e}");
    });

    let fox = Fox::new(&api_key, &sn, 30).unwrap_or_else(|e| {
        panic!("Failed to create Fox instance: {e}");
    });

    let bat_temp = fox.get_variable_typed::<BatTemperature>().await.unwrap_or_else(|e| {
        panic!("Failed to get device variables: {e}");
    });
    println!("{:?}", bat_temp);
}

#[cfg(feature = "blocking")]
#[test]
fn it_works() {
    let api_key = read_credential("fox_ess_api_key").unwrap_or_else(|e| {
        panic!("fox_ess_api_key not found in credstore: {e}");
    });
    let sn = read_credential("fox_ess_inverter_sn").unwrap_or_else(|e| {
        panic!("fox_ess_inverter_sn not found in credstore: {e}");
    });

    let fox = Fox::new(&api_key, &sn, 30).unwrap_or_else(|e| {
        panic!("Failed to create Fox instance: {e}");
    });

    let bat_temp = fox.get_variable_typed::<BatTemperature>().unwrap_or_else(|e| {
        panic!("Failed to get device variables: {e}");
    });
    println!("{:?}", bat_temp);
}

/// Reads a credential from the file system supported by the credstore and
/// given from systemd
///
/// # Arguments
///
/// * 'name' - name of the credential to read
fn read_credential(name: &str) -> Result<String, ConfigError> {
    let dir = env::var("CREDENTIALS_DIRECTORY")?;
    let mut p = PathBuf::from(dir);
    p.push(name);
    let bytes = fs::read(p)?;
    Ok(String::from_utf8(bytes)?.trim_end().to_string())
}

/// Errors while managing configuration
///
#[derive(Debug, Error)]
enum ConfigError {
    #[error("IoError: {0}")]
    IoError(#[from] std::io::Error),
    #[error("StringConversionError: {0}")]
    StringConversionError(#[from] alloc::string::FromUtf8Error),
    #[error("EnvVarError: {0}")]
    EnvVarError(#[from] env::VarError),
}
