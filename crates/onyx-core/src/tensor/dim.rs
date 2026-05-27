#[non_exhaustive]
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "lowercase")]
pub enum Dim {
    Known(usize),
    Symbol(std::borrow::Cow<'static, str>),
    #[default]
    #[serde(other)]
    Unknown,
}

impl Dim {
    pub const fn known(value: usize) -> Self {
        Self::Known(value)
    }

    pub fn symbol(value: impl Into<std::borrow::Cow<'static, str>>) -> Self {
        Self::Symbol(value.into())
    }

    pub const fn is_known(&self) -> bool {
        matches!(self, Self::Known(_))
    }

    pub const fn as_usize(&self) -> Option<usize> {
        match self {
            Self::Known(value) => Some(*value),
            Self::Symbol(_) | Self::Unknown => None,
        }
    }
}

impl std::fmt::Display for Dim {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Known(size) => write!(f, "{size}"),
            Self::Symbol(sym) => write!(f, "{sym}"),
            _ => write!(f, "??"),
        }
    }
}
