//! Exported API models for the FoxESS Main Switch Status.
//!
//! This module contains types that represent the Main Switch Status state
//! in the API, providing a more convenient representation compared to the raw DTOs.
//! 


/// Represents the Main Switch Status state in the API.
#[derive(Debug)]
pub struct MainSwitchStatus {
    /// Whether the main switch is supported (true) or not (false).
    pub support: bool,
    /// Whether the main switch is enabled (true) or disabled (false).
    pub enable: bool,
}