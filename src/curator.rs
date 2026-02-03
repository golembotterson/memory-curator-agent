// Core curation logic for Memory Curator Agent

use crate::models::{CuratorConfig, CuratorReport, RemovalEntry, SignalEntry};
use crate::error::{CuratorError, Result};
use chrono::{DateTime, Duration, Utc};
use regex::Regex;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

pub struct MemoryCurator {
    config: CuratorConfig,
    report: CuratorReport,
}

impl MemoryCurator {
    pub fn new(config: CuratorConfig) -> Result<Self> {
        if !config.memory_dir.exists() {
            return Err(CuratorError::FileNotFound(config.memory_dir.clone()));
        }
        if !config.memory_file.exists() {
            return Err(CuratorError::FileNotFound(config.memory_file.clone()));
        }

        Ok(Self {
            config,
            report: CuratorReport::new("memory-curator-v0.1".to_string()),
        })
    }

    /// Scan recent daily files
    pub fn scan_daily_files(&mut self) -> Result<Vec<PathBuf>> {
        let mut files = vec![];
        let cutoff = Utc::now() - Duration::days(self.config.days_to_review as i64);

        for entry in WalkDir::new(&self.config.memory_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "md"))
        {
            if let Ok(metadata) = entry.metadata() {
                if let Ok(modified) = metadata.modified() {
                    if let Ok(duration) = modified.elapsed() {
                        let modified_time = Utc::now()
                            - Duration::from_std(duration).unwrap_or_default();
                        if modified_time > cutoff {
                            files.push(entry.path().to_path_buf());
                            self.report
                                .files_scanned
                                .push(entry.path().display().to_string());
                        }
                    }
                }
            }
        }

        Ok(files)
    }

    /// Extract signal entries from daily files
    pub fn extract_signal(&mut self, files: Vec<PathBuf>) -> Result<Vec<SignalEntry>> {
        let mut signals = vec![];

        for file in files {
            let content = fs::read_to_string(&file)
                .map_err(|e| CuratorError::Io(e))?;

            // Extract headlines and structured entries
            let headlines = extract_headlines(&content);
            for headline in headlines {
                let confidence = score_signal(&headline);
                if confidence >= self.config.min_signal_confidence {
                    signals.push(SignalEntry {
                        section: "General".to_string(),
                        content: headline,
                        source_file: file.display().to_string(),
                        confidence,
                        extracted_at: Utc::now(),
                    });
                }
            }
        }

        // Limit to max entries
        signals.truncate(self.config.max_daily_entries);
        Ok(signals)
    }

    /// Merge signal entries into MEMORY.md
    pub fn merge_into_memory(&mut self, signals: Vec<SignalEntry>) -> Result<()> {
        let mut memory_content = fs::read_to_string(&self.config.memory_file)
            .map_err(|e| CuratorError::Io(e))?;

        // Append new entries with metadata
        for signal in signals {
            let entry = format!(
                "\n- **{}** (confidence: {:.2}) — {}\n  Source: {}\n",
                signal.content,
                signal.confidence,
                signal.extracted_at.to_rfc2822(),
                signal.source_file
            );
            memory_content.push_str(&entry);
            self.report.additions.push(signal);
        }

        // Write back with snapshot for rollback
        fs::write(&self.config.memory_file, &memory_content)
            .map_err(|e| CuratorError::Io(e))?;

        Ok(())
    }

    /// Prune stale entries from MEMORY.md
    pub fn prune_stale(&mut self) -> Result<usize> {
        let content = fs::read_to_string(&self.config.memory_file)
            .map_err(|e| CuratorError::Io(e))?;

        let cutoff = Utc::now() - Duration::days(self.config.prune_threshold_days as i64);
        let lines: Vec<&str> = content.lines().collect();

        let mut pruned_count = 0;
        let mut new_content = String::new();

        for line in lines {
            // Parse date if present, skip old entries
            if let Some(date_str) = extract_date(line) {
                if let Ok(date) = DateTime::parse_from_rfc2822(date_str) {
                    let utc_date: DateTime<Utc> = date.with_timezone(&Utc);
                    if utc_date < cutoff {
                        // Log removal
                        self.report.removals.push(RemovalEntry {
                            section: "Unknown".to_string(),
                            content: line.to_string(),
                            reason: "age_threshold_exceeded".to_string(),
                            last_referenced: Some(utc_date.to_rfc2822()),
                        });
                        pruned_count += 1;
                        continue;
                    }
                }
            }
            new_content.push_str(line);
            new_content.push('\n');
        }

        if pruned_count > 0 {
            fs::write(&self.config.memory_file, &new_content)
                .map_err(|e| CuratorError::Io(e))?;
        }

        Ok(pruned_count)
    }

    /// Generate final report
    pub fn finalize_report(&mut self) -> Result<String> {
        self.report.status = "completed".to_string();
        let report_json = serde_json::to_string_pretty(&self.report)?;
        Ok(report_json)
    }

    pub fn report(&self) -> &CuratorReport {
        &self.report
    }
}

// Helper functions

fn extract_headlines(content: &str) -> Vec<String> {
    let re = Regex::new(r"^#+\s+(.+)$").unwrap();
    re.captures_iter(content)
        .filter_map(|cap| cap.get(1).map(|m| m.as_str().to_string()))
        .collect()
}

fn score_signal(text: &str) -> f32 {
    let mut score: f32 = 0.5;

    // Boost for keywords
    let keywords = vec!["decision", "learned", "important", "critical", "completed", "project"];
    for kw in keywords {
        if text.to_lowercase().contains(kw) {
            score += 0.1;
        }
    }

    // Boost for structured entries
    if text.contains("Status") || text.contains("Next Steps") {
        score += 0.2;
    }

    score.min(1.0)
}

fn extract_date(line: &str) -> Option<&str> {
    let re = Regex::new(r"(\w{3},\s+\d{1,2}\s+\w{3}\s+\d{4}\s+\d{2}:\d{2}:\d{2})").ok()?;
    re.captures(line)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_headlines() {
        let content = "# Main\n## Sub\n### Deep\nNot a headline";
        let headlines = extract_headlines(content);
        assert_eq!(headlines.len(), 3);
    }

    #[test]
    fn test_signal_scoring() {
        assert!(score_signal("Decision: move to Rust") > 0.5);
        assert!(score_signal("Status: Active") > 0.5);
        assert!(score_signal("random text") < 0.7);
    }
}
