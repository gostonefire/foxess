use crate::fox_settings::SettingSpec;

/// A trait for settings that can be updated on the FoxESS cloud.
///
/// Types implementing this trait provide a way to format a value back into a string
/// for API requests.
pub trait SettableSettingSpec: SettingSpec {
    /// Formats the value into a string suitable for an API request.
    ///
    /// # Arguments
    /// * `value` - A reference to the value to be formatted.
    ///
    /// # Returns
    /// * `String` - The formatted string.
    fn format(value: &Self::Value) -> String;
}
