extern crate alloc;
use std::{env, fs};
use std::path::PathBuf;
use thiserror::Error;
use foxess::Fox;

#[cfg(feature = "async")]
#[tokio::test]
/// Basic integration test for asynchronous functionality.
///
/// This test verifies that the `Fox` client can be successfully initialized and
/// that a typed variable (Battery Temperature) can be retrieved using the async API.
///
/// # Panics
/// * If the `fox_ess_api_key` or `fox_ess_inverter_sn` credentials are missing.
/// * If the `Fox` instance fails to initialize.
/// * If fetching the battery temperature fails.
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

    // Put the latest tested development in integration here:
    let x = fox.get_error_code_information().await.unwrap_or_else(|e| {
        panic!("Failed to get error code information: {e}");
    });

    let mut codes = x.keys().collect::<Vec<&u32>>();
    codes.sort();

    for c in codes {
        println!("{} - {}", c, x.get(c).unwrap_or(&"Unknown".to_string()) );
    }
    // End latest tested development!

    /*

    let mut variables = fox.get_available_variables().await.unwrap_or_else(|e| {
        panic!("Failed to get available variables: {e}");
    });

    variables.variables.sort_by_key(|v| v.variable.to_ascii_lowercase());

    for v in variables.variables {
        let mut chars = v.variable.chars();
        let variant = match chars.next() {
            Some(first) => format!("{}{}", first.to_uppercase(), chars.as_str()),
            None => String::new(),
        };

        let unit_suffix = v.unit.as_deref().map_or(String::new(), |u| format!(", unit: {u}"));
        let enum_suffix = if v.enumeration.is_some() {
            let mut items: Vec<String> = v.enumeration.unwrap()
                .iter()
                .map(|(k, v)| format!(r#""{}" => "{}""#, k, v))
                .collect();
            items.sort();
            let joined = items.join(", ");

            format!("; values: {{ {joined} }}")
        } else { "".to_string() };

        if !variant.is_empty() {
            println!(r#"[{variant}, "{}", "{}{}"{}],"#, v.variable, v.name, unit_suffix, enum_suffix);
        }
    }

    */
}

#[cfg(feature = "blocking")]
#[test]
/// Basic integration test for blocking functionality.
///
/// This test verifies that the `Fox` client can be successfully initialized and
/// that a typed variable (Battery Temperature) can be retrieved using the blocking API.
///
/// # Panics
/// * If the `fox_ess_api_key` or `fox_ess_inverter_sn` credentials are missing.
/// * If the `Fox` instance fails to initialize.
/// * If fetching the battery temperature fails.
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

    // Put the latest tested development in integration here:
    let x = fox.get_error_code_information().unwrap_or_else(|e| {
        panic!("Failed to get error code information: {e}");
    });

    let mut codes = x.keys().collect::<Vec<&u32>>();
    codes.sort();

    for c in codes {
        println!("{} - {}", c, x.get(c).unwrap_or(&"Unknown".to_string()) );
    }
    // End latest tested development!

    /*

    let mut variables = fox.get_available_variables().unwrap_or_else(|e| {
        panic!("Failed to get available variables: {e}");
    });

    variables.variables.sort_by_key(|v| v.variable.to_ascii_lowercase());

    for v in variables.variables {
        let mut chars = v.variable.chars();
        let variant = match chars.next() {
            Some(first) => format!("{}{}", first.to_uppercase(), chars.as_str()),
            None => String::new(),
        };

        let unit_suffix = v.unit.as_deref().map_or(String::new(), |u| format!(", unit: {u}"));
        let enum_suffix = if v.enumeration.is_some() {
            let mut items: Vec<String> = v.enumeration.unwrap()
                .iter()
                .map(|(k, v)| format!(r#""{}" => "{}""#, k, v))
                .collect();
            items.sort();
            let joined = items.join(", ");

            format!("; values: {{ {joined} }}")
        } else { "".to_string() };

        if !variant.is_empty() {
            println!(r#"[{variant}, "{}", "{}{}"{}],"#, v.variable, v.name, unit_suffix, enum_suffix);
        }
    }

    */
}

/// Reads a credential from the file system.
///
/// This function reads credentials stored in the directory specified by the
/// `CREDENTIALS_DIRECTORY` environment variable, which is commonly used
/// by `systemd-creds`.
///
/// # Arguments
/// * `name` - The name of the credential to read.
///
/// # Returns
/// * `Result<String, ConfigError>` - The credential value as a trimmed string, or an error if reading fails.
fn read_credential(name: &str) -> Result<String, ConfigError> {
    let dir = env::var("CREDENTIALS_DIRECTORY")?;
    let mut p = PathBuf::from(dir);
    p.push(name);
    let bytes = fs::read(p)?;
    Ok(String::from_utf8(bytes)?.trim_end().to_string())
}

/// Errors that can occur while managing configuration and credentials.
#[derive(Debug, Error)]
enum ConfigError {
    /// An I/O error occurred while reading the credential file.
    #[error("IoError: {0}")]
    IoError(#[from] std::io::Error),
    /// The credential file contained invalid UTF-8 data.
    #[error("StringConversionError: {0}")]
    StringConversionError(#[from] alloc::string::FromUtf8Error),
    /// The `CREDENTIALS_DIRECTORY` environment variable is not set.
    #[error("EnvVarError: {0}")]
    EnvVarError(#[from] env::VarError),
}
