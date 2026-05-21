#[derive(Debug, Copy, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Architecture {
    #[serde(rename = "BERT")]
    Bert,

    #[serde(rename = "RoBERTa")]
    Roberta,

    #[serde(rename = "DistilBERT")]
    DistilBert,

    #[serde(rename = "DeBERTa")]
    Deberta,
}

impl Architecture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Bert => "BERT",
            Self::Roberta => "RoBERTa",
            Self::DistilBert => "DistilBERT",
            Self::Deberta => "DeBERTa",
        }
    }
}

impl std::fmt::Display for Architecture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
