use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use serde::Deserialize;

/// Represents a config containing the details of a key combo.
#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    /// List of keys to hit in sequence when the combo is activated.
    pub combo: Vec<u64>,

    /// Delay between each key press.
    pub delay: u64,

    /// Description of this config.
    pub description: String,

    /// List of modifier keys to hold down before pressing the trigger.
    pub modifiers: Vec<u64>,

    /// Whether the combo should run repeatedly.
    #[serde(default)]
    pub repeat: bool,

    /// Trigger key to trigger the key combo.
    pub trigger: u64
}

/// Returns configs from a JSON config file.
///
/// # Arguments
/// - `path`: Specifies the path to the config file.
///
/// # Return
/// The parsed configs from the config file or error.
pub fn get_file_config(path: &str) -> Result<Vec<Config>, Box<dyn Error>> {
    let reader = BufReader::new(File::open(path)?);
    Ok(serde_json::from_reader(reader)?)
}

/// Returns configs from a network file.
///
/// # Arguments
/// - `url`: Specifies the URL to the webpage.
///
/// # Return
/// The parsed configs from the network config or error.
pub fn get_network_config(url: &str) -> Result<Vec<Config>, Box<dyn Error>> {
    Ok(reqwest::blocking::get(url)?.json()?)
}
