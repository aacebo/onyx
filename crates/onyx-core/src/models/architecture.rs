#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ModelArchitecture {
    BertForMaskedLM,
    BertForSequenceClassification,
    BertForTokenClassification,
    BertForQuestionAnswering,
    BertModel,
}

impl ModelArchitecture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::BertForMaskedLM => "BertForMaskedLM",
            Self::BertForSequenceClassification => "BertForSequenceClassification",
            Self::BertForTokenClassification => "BertForTokenClassification",
            Self::BertForQuestionAnswering => "BertForQuestionAnswering",
            Self::BertModel => "BertModel",
        }
    }
}

impl std::fmt::Display for ModelArchitecture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
