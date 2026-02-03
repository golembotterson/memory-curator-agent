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
    fn test_curator_init() {
        // Basic initialization test
        let config = CuratorConfig::default();
        assert!(config.memory_dir.exists());
    }
}
