use crate::conversion::integrations::{evaluated_integration_approaches, IntegrationApproach};
use crate::conversion::types::ConversionJobKind;

#[derive(Debug, Clone)]
pub struct ConversionRegistry {
    approaches: Vec<IntegrationApproach>,
}

impl Default for ConversionRegistry {
    fn default() -> Self {
        Self {
            approaches: evaluated_integration_approaches(),
        }
    }
}

impl ConversionRegistry {
    pub fn approaches(&self) -> &[IntegrationApproach] {
        &self.approaches
    }

    pub fn approaches_for(&self, kind: &ConversionJobKind) -> Vec<&IntegrationApproach> {
        self.approaches
            .iter()
            .filter(|approach| approach.best_for.iter().any(|candidate| candidate == kind))
            .collect()
    }
}
