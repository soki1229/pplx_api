#[derive(Debug, Clone)]
pub enum Model {
    SonarSmall,
    SonarLarge,
    SonarHuge
}

impl Model {
    pub fn as_str(&self) -> &'static str {
        match self {
            Model::SonarSmall => "llama-3.1-sonar-small-128k-online",
            Model::SonarLarge => "llama-3.1-sonar-large-128k-online",
            Model::SonarHuge => "llama-3.1-sonar-huge-128k-online",
        }
    }
}
