extern crate alloc;
use std::{env, fs};
use std::path::PathBuf;
use thiserror::Error;
use foxess::{Fox, WorkMode, MinSocOnGrid, MaxSetChargeCurrent};

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

    let work_mode = fox.get_setting_typed::<WorkMode>().await.unwrap_or_else(|e| {
        panic!("Failed to get device settings: {e}");
    });
    println!("{}", work_mode);

    let max_set_charge_current = fox.get_setting_typed::<MaxSetChargeCurrent>().await.unwrap_or_else(|e| {
        panic!("Failed to get device settings: {e}");
    });
    println!("{:?}", max_set_charge_current);

    let _ = fox.set_setting_typed::<MinSocOnGrid>(10).await.unwrap_or_else(|e| {
        panic!("Failed to set device settings: {e}");
    });
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

    let work_mode = fox.get_setting_typed::<WorkMode>().unwrap_or_else(|e| {
        panic!("Failed to get device settings: {e}");
    });
    println!("{}", work_mode);

    let max_set_charge_current = fox.get_setting_typed::<MaxSetChargeCurrent>().unwrap_or_else(|e| {
        panic!("Failed to get device settings: {e}");
    });
    println!("{:?}", max_set_charge_current);

    let _ = fox.set_setting_typed::<MinSocOnGrid>(10).unwrap_or_else(|e| {
        panic!("Failed to set device settings: {e}");
    });
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
