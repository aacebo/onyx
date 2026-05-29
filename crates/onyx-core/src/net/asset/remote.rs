#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize)]
pub struct Remote {
    base_url: url::Url,

    #[serde(skip)]
    url: url::Url,

    #[serde(flatten)]
    asset: Box<super::Asset>,
}

impl Remote {
    pub fn from_asset(base_url: url::Url, asset: super::Asset) -> Result<Self, crate::OnyxError> {
        let path = asset.path().to_str().ok_or(crate::error::ParseError::InvalidUri(format!(
            "invalid asset path: `{}`",
            asset.path().display()
        )))?;

        let url = base_url.join(path)?;

        Ok(Self {
            base_url,
            url,
            asset: Box::new(asset),
        })
    }

    pub fn base_url(&self) -> &url::Url {
        &self.base_url
    }

    pub fn url(&self) -> &url::Url {
        &self.url
    }

    pub fn asset(&self) -> &super::Asset {
        &self.asset
    }
}

impl std::ops::Deref for Remote {
    type Target = super::Asset;

    fn deref(&self) -> &Self::Target {
        &self.asset
    }
}

impl<'de> serde::Deserialize<'de> for Remote {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(serde::Deserialize)]
        struct _Remote {
            base_url: url::Url,
            #[serde(flatten)]
            asset: super::Asset,
        }

        let data = _Remote::deserialize(deserializer)?;
        Remote::from_asset(data.base_url, data.asset).map_err(serde::de::Error::custom)
    }
}
