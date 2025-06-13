use crate::error::PollenError;
use serde_yaml::{Mapping, Value};

/// Extension trait for serde_yaml::Value to provide convenient string conversion
pub trait CanForceIntoString {
    fn force_into_string(&self) -> Result<String, PollenError>;
}

impl CanForceIntoString for Value {
    fn force_into_string(&self) -> Result<String, PollenError> {
        match self {
            Value::String(s) => Ok(s.to_string()),
            _ => Err(PollenError::InvalidMapping(format!(
                "Expected a string value, got {:?}", 
                self
            ))),
        }
    }
}

/// Extension trait for serde_yaml::Mapping to determine endpoints and extract data
pub trait Endpoint {
    /// Determine if a mapping represents an endpoint or a directory to recurse into
    fn is_an_endpoint(&self) -> Result<bool, PollenError>;
    /// Get the single key from a mapping (for endpoints)
    fn get_key(&self) -> Result<String, PollenError>;
    /// Get the single value from a mapping (for endpoints)
    fn get_value(&self) -> Result<&Value, PollenError>;
}

impl Endpoint for Mapping {
    fn is_an_endpoint(&self) -> Result<bool, PollenError> {
        // An endpoint will always have a single key
        if self.keys().len() != 1 {
            return Ok(false);
        }

        // An endpoint will always have a single value of a `Sequence`
        if self.values().len() != 1 {
            return Ok(false);
        }

        match self.values().next() {
            Some(value) => {
                if !value.is_sequence() {
                    return Ok(false);
                }

                let sequence = value.as_sequence().unwrap();
                
                // If every submapping is a valid option, this is an endpoint
                for (_index, mapping) in sequence.iter().enumerate() {
                    if !mapping.is_mapping() {
                        return Ok(false);
                    }

                    let mapping = mapping.as_mapping().unwrap();
                    for key in mapping.keys() {
                        if !key.is_string() {
                            return Err(PollenError::InvalidEndpoint(format!(
                                "Expected a string key, got {:?}", key
                            )));
                        }

                        const VALID_OPTIONS: [&str; 4] = ["run_before", "run_after", "depends_on", "alias_as"];
                        let key_str = key.force_into_string()?;
                        let is_valid_option = VALID_OPTIONS.contains(&key_str.as_str());

                        if !is_valid_option {
                            return Ok(false);
                        }
                    }
                }
                Ok(true)
            }
            None => Ok(false),
        }
    }

    fn get_key(&self) -> Result<String, PollenError> {
        if self.keys().len() != 1 {
            return Err(PollenError::InvalidMapping(format!(
                "Mapping has {} keys, expected exactly 1", 
                self.keys().len()
            )));
        }

        let key = self.keys().next().unwrap();
        key.force_into_string()
    }

    fn get_value(&self) -> Result<&Value, PollenError> {
        if self.values().len() != 1 {
            return Err(PollenError::InvalidMapping(format!(
                "Mapping has {} values, expected exactly 1", 
                self.values().len()
            )));
        }

        Ok(self.values().next().unwrap())
    }
}
