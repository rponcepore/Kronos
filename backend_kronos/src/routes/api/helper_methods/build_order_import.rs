//! order_builder.rs
//!

use crate::routes::api::parameters::network_structs::KronosApiError;
use serde::{Deserialize, Serialize};
use std::fs;

use std::path::PathBuf;


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ImportOrder {
    pub paragraphs: Vec<ImportParagraph>,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ImportParagraph {
    pub ordinal_sequence: i32,
    pub title: String,
    pub text: String,
    pub subparagraphs: Vec<ImportParagraph>,
}

pub fn make_standard_order() -> Result<ImportOrder, KronosApiError> {
    let path = get_opord_path()?;
    let yaml_str = match fs::read_to_string(&path) {
        Ok(yaml_str) => yaml_str,
        Err(msg) => {
            return Err(KronosApiError::Unknown(format!(
                "Failed to read file at {}, Error Message: {}",
                path.display(), msg
            )))
        }
    };
    let opord: ImportOrder = match serde_yaml::from_str(&yaml_str) {
        // crate deprecated but unlikely to change soon.
        Ok(opord) => opord,
        Err(msg) => {
            return Err(KronosApiError::Unknown(format!(
                "Failed to serialize string to ImportOrder struct. Error: {}, &str: {}",
                msg, yaml_str
            )))
        }
    };

    println!("{:#?}", opord);
    Ok(opord)
}


pub fn make_standard_fragord() -> Result<ImportOrder, KronosApiError> {
    let path = get_fragord_path()?;
    let yaml_str = match fs::read_to_string(&path) {
        Ok(yaml_str) => yaml_str,
        Err(msg) => {
            return Err(KronosApiError::Unknown(format!(
                "Failed to read file at {}, Error Message: {}",
                path.display(), msg
            )))
        }
    };
    let fragord: ImportOrder = match serde_yaml::from_str(&yaml_str) {
        // crate deprecated but unlikely to change soon.
        Ok(opord) => opord,
        Err(msg) => {
            return Err(KronosApiError::Unknown(format!(
                "Failed to serialize string to ImportOrder struct. Error: {}, &str: {}",
                msg, yaml_str
            )))
        }
    };

    println!("{:#?}", fragord);
    Ok(fragord)
}

// Can't declare it as a constant, so instead we'll get it dynamically. 
fn get_opord_path() -> Result<PathBuf, KronosApiError> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let manifest_path = PathBuf::from(manifest_dir);

    let parent = manifest_path.parent().ok_or_else(|| {
        KronosApiError::Unknown("Failed to get parent directory of CARGO_MANIFEST_DIR".to_string())
    })?;

    Ok(parent.join("configs/standard_opord_contents.yaml"))
}


fn get_fragord_path() -> Result<PathBuf, KronosApiError> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let manifest_path = PathBuf::from(manifest_dir);

    let parent = manifest_path.parent().ok_or_else(|| {
        KronosApiError::Unknown("Failed to get parent directory of CARGO_MANIFEST_DIR".to_string())
    })?;

    Ok(parent.join("configs/standard_fragord_contents.yaml"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_make_standard_order_success() {
        let result = make_standard_order();
        assert!(result.is_ok(), "Expected make_standard_order to succeed, but it failed: {:?}", result.err());
        
        let order = result.unwrap();
        
        assert_eq!(order.paragraphs.len(), 5);
        println!("{:#?}", order);
    }

    #[test]
    fn test_make_standard_fragord_success() {
        let result = make_standard_fragord();
        assert!(result.is_ok(), "Expected make_standard_order to succeed, but it failed: {:?}", result.err());
        
        let fragord = result.unwrap();
        
        assert_eq!(fragord.paragraphs.len(), 5);
        println!("{:#?}", fragord);
    }

    #[test]
    fn test_get_fragord_path_success() {
        let path_result = get_fragord_path();
        assert!(path_result.is_ok(), "Expected Ok(PathBuf), got Err: {:?}", path_result);

        let path = path_result.unwrap();
        println!("Resolved path: {:?}", path);

        // Optionally check if the file exists
        assert!(
            path.exists(),
            "Expected path to exist: {:?}, but it doesn't.",
            path
        );

        // Optionally, you can check if it's a file
        assert!(
            path.is_file(),
            "Expected path to be a file: {:?}, but it's not.",
            path
        );
    }

    #[test]
    fn test_get_opord_path_success() {
        let path_result = get_opord_path();
        assert!(path_result.is_ok(), "Expected Ok(PathBuf), got Err: {:?}", path_result);

        let path = path_result.unwrap();
        println!("Resolved path: {:?}", path);

        // Optionally check if the file exists
        assert!(
            path.exists(),
            "Expected path to exist: {:?}, but it doesn't.",
            path
        );

        // Optionally, you can check if it's a file
        assert!(
            path.is_file(),
            "Expected path to be a file: {:?}, but it's not.",
            path
        );
    }
}


