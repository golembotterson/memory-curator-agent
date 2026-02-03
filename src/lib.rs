// Memory Curator Agent Library
// Core logic for scanning, curating, and maintaining memory files

pub mod curator;
pub mod error;
pub mod models;

pub use curator::MemoryCurator;
pub use error::{CuratorError, Result};
pub use models::{CuratorConfig, CuratorReport, SignalEntry};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_curator_config_default() {
        // Verify default config structure
        let config = CuratorConfig::default();
        assert_eq!(config.days_to_review, 2);
        assert_eq!(config.max_daily_entries, 5);
        assert_eq!(config.min_signal_confidence, 0.7);
        assert_eq!(config.prune_threshold_days, 90);
    }
}
