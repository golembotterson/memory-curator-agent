// Data models for Memory Curator

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CuratorConfig {
    pub memory_dir: PathBuf,
    pub days_to_review: usize,
    pub memory_file: PathBuf,
    pub max_daily_entries: usize,
    pub min_signal_confidence: f32,
    pub prune_threshold_days: i32,
}

impl Default for CuratorConfig {
    fn default() -> Self {
        Self {
            memory_dir: PathBuf::from("/home/jascha/.openclaw/workspace/memory"),
            days_to_review: 2,
            memory_file: PathBuf::from("/home/jascha/.openclaw/workspace/MEMORY.md"),
            max_daily_entries: 5,
            min_signal_confidence: 0.7,
            prune_threshold_days: 90,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalEntry {
    pub section: String,
    pub content: String,
    pub source_file: String,
    pub confidence: f32,
    pub extracted_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemovalEntry {
    pub section: String,
    pub content: String,
    pub reason: String,
    pub last_referenced: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CuratorReport {
    pub timestamp: DateTime<Utc>,
    pub agent_id: String,
    pub files_scanned: Vec<String>,
    pub additions: Vec<SignalEntry>,
    pub removals: Vec<RemovalEntry>,
    pub status: String,
    pub error: Option<String>,
}

impl CuratorReport {
    pub fn new(agent_id: String) -> Self {
        Self {
            timestamp: Utc::now(),
            agent_id,
            files_scanned: vec![],
            additions: vec![],
            removals: vec![],
            status: "initialized".to_string(),
            error: None,
        }
    }

    pub fn summary(&self) -> String {
        format!(
            "Curator: {} files scanned, {} added, {} removed",
            self.files_scanned.len(),
            self.additions.len(),
            self.removals.len()
        )
    }
}
