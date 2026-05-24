//! Training focus definitions used by the local MVP loop.

/// Targeted training programs available in the laboratory.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrainingFocus {
    Endurance,
    Power,
    Guard,
    Reflex,
}

impl TrainingFocus {
    pub fn all() -> [TrainingFocus; 4] {
        [
            TrainingFocus::Endurance,
            TrainingFocus::Power,
            TrainingFocus::Guard,
            TrainingFocus::Reflex,
        ]
    }

    pub fn label(self) -> &'static str {
        match self {
            TrainingFocus::Endurance => "Endurance",
            TrainingFocus::Power => "Power",
            TrainingFocus::Guard => "Guard",
            TrainingFocus::Reflex => "Reflex",
        }
    }

    pub fn stat_label(self) -> &'static str {
        match self {
            TrainingFocus::Endurance => "HP",
            TrainingFocus::Power => "Attack",
            TrainingFocus::Guard => "Defense",
            TrainingFocus::Reflex => "Speed",
        }
    }
}
