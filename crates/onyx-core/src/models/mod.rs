mod architecture;
pub mod bert;
mod config;
mod r#type;

pub use architecture::*;
pub use config::*;
pub use r#type::*;

use crate::error;

#[derive(Clone, PartialEq, Eq)]
pub struct ModelId {
    group: Option<Box<str>>,
    name: Box<str>,
}

impl ModelId {
    pub fn group(&self) -> Option<&str> {
        self.group.as_deref()
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl std::str::FromStr for ModelId {
    type Err = error::ModelError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains("/") {
            let (group, name) = match s.split_once("/") {
                None => return Err(error::ModelError::parse("invalid model id format")),
                Some(v) => v,
            };

            if group.is_empty() || name.is_empty() {
                return Err(error::ModelError::parse("invalid model id format"));
            }

            return Ok(Self {
                group: Some(group.into()),
                name: name.into(),
            });
        }

        if s.is_empty() {
            return Err(error::ModelError::parse("invalid model id format"));
        }

        Ok(Self {
            group: None,
            name: s.into(),
        })
    }
}

impl std::fmt::Debug for ModelId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::fmt::Display for ModelId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(group) = &self.group {
            return write!(f, "{}/{}", group, &self.name);
        }

        write!(f, "{}", &self.name)
    }
}

impl serde::Serialize for ModelId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for ModelId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::str::FromStr;

        let value = String::deserialize(deserializer)?;
        Self::from_str(&value).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn parse_valid() {
        let id = ModelId::from_str("facebook/bart-large").expect("should parse");
        assert_eq!(id.group(), Some("facebook"));
        assert_eq!(id.name(), "bart-large");
        assert_eq!(id.to_string(), "facebook/bart-large");
    }

    #[test]
    fn parse_nested_slash() {
        let id = ModelId::from_str("facebook/bart/large").expect("should parse");
        assert_eq!(id.group(), Some("facebook"));
        assert_eq!(id.name(), "bart/large");
        assert_eq!(id.to_string(), "facebook/bart/large");
    }

    #[test]
    fn parse_empty_segments() {
        for input in ["/name", "group/", "/", ""] {
            let err = ModelId::from_str(input).expect_err("should fail");
            assert!(
                matches!(err, error::ModelError::Parse(_)),
                "expected Parse error for {input:?}"
            );
        }
    }

    #[test]
    fn display_roundtrip() {
        let id = ModelId::from_str("facebook/bart-large").unwrap();
        assert_eq!(format!("{id}"), "facebook/bart-large");
        assert_eq!(format!("{id:?}"), "facebook/bart-large");
    }

    #[test]
    fn serde_roundtrip() {
        let id = ModelId::from_str("facebook/bart-large").unwrap();
        let json = serde_json::to_string(&id).unwrap();
        assert_eq!(json, "\"facebook/bart-large\"");

        let back: ModelId = serde_json::from_str(&json).unwrap();
        assert_eq!(back, id);

        assert_eq!(
            serde_json::from_str::<ModelId>("\"nogroup\"")
                .unwrap()
                .to_string(),
            "nogroup"
        );
    }
}
