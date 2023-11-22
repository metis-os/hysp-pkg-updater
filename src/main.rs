use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Available {
    pub packages: Vec<Packagefields>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Packagefields {
    pub name: String,
    pub binary_size: Option<String>,
    pub maintainer: Option<String>,
    pub email: Option<String>,
    pub description: Option<String>,
    pub version: Option<String>,
    pub homepage: Option<String>,
    pub license: Option<String>,
}

fn parse_toml(file_path: &str) -> Result<Available, Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;
    let available: Available = toml::from_str(&contents)?;

    Ok(available)
}

fn update_toml_with_defaults(original_path: &str, new_path: &str) -> Result<(), Box<dyn Error>> {
    let available = parse_toml(original_path)?;

    // Create a new Available instance with modified content
    let mut updated_available = available.clone();
    for package in &mut updated_available.packages {
        if package.binary_size.as_deref() == Some("") {
            package.binary_size = Some("Not provided".to_string());
        }
        if package.maintainer.as_deref() == Some("") {
            package.maintainer = Some("hysp bot".to_string());
        }
        if package.email.as_deref() == Some("") {
            package.email = Some("bot@hysp-noreply.com".to_string());
        }
        if package.description.as_deref() == Some("") {
            package.description = Some("Not provided".to_string());
        }
        if package.version.as_deref() == Some("") {
            package.version = Some("Latest".to_string());
        }
        if package.homepage.as_deref() == Some("") {
            package.homepage = Some("Not provided".to_string());
        }
        if package.license.as_deref() == Some("") {
            package.license = Some("Not provided".to_string());
        }
    }

    let toml_content = toml::to_string_pretty(&updated_available)?;
    fs::write(new_path, toml_content)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let original_file_path = "available.toml";
    let new_file_path = "available-bk.toml";

    update_toml_with_defaults(original_file_path, new_file_path)?;

    println!("Created a new file with 'not provided' values added.");

    Ok(())
}
