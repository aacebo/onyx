use crate::{error, model};

#[derive(Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct HFResource {
    model_id: model::ModelId,
    filename: String,
    revision: Option<String>,
}

impl HFResource {
    pub fn new(
        model_id: model::ModelId,
        filename: impl Into<String>,
        revision: Option<impl Into<String>>,
    ) -> Self {
        Self {
            model_id,
            filename: filename.into(),
            revision: revision.map(|rev| rev.into()),
        }
    }

    pub fn model_id(&self) -> &model::ModelId {
        &self.model_id
    }

    pub fn filename(&self) -> &str {
        &self.filename
    }

    pub fn revision(&self) -> Option<&str> {
        self.revision.as_deref()
    }
}

impl std::str::FromStr for HFResource {
    type Err = error::ResourceError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut parts = value.trim_start_matches("hf://").split('#');
        let model_id = parts.next().unwrap_or_default();
        let filename = parts.next().ok_or(error::ResourceError::parse(
            "invalid resource id format: missing filename",
        ))?;
        let revision = parts.next();

        if parts.next().is_some() {
            return Err(error::ResourceError::parse(
                "invalid resource id format: too many fragments",
            ));
        }

        if model_id.is_empty() {
            return Err(error::ResourceError::parse(
                "invalid resource id format: missing model id",
            ));
        }

        if filename.is_empty() {
            return Err(error::ResourceError::parse(
                "invalid resource id format: missing filename",
            ));
        }

        Ok(Self {
            model_id: model_id.parse().map_err(error::ResourceError::parse)?,
            filename: filename.to_string(),
            revision: revision
                .filter(|value| !value.is_empty())
                .map(ToOwned::to_owned),
        })
    }
}

impl std::fmt::Debug for HFResource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::fmt::Display for HFResource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "hf://{}#{}", &self.model_id, &self.filename)?;

        if let Some(rev) = &self.revision {
            write!(f, "#{}", rev)?;
        }

        Ok(())
    }
}
