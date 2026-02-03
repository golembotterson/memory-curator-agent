// Memory Curator Agent - CLI Entry Point

use memory_curator_agent::{MemoryCurator, CuratorConfig};
use std::path::PathBuf;
use clap::Parser;
use anyhow::Result;

#[derive(Parser)]
#[command(name = "curator")]
#[command(about = "Memory Curator Agent - curate daily logs into MEMORY.md", long_about = None)]
struct Args {
    /// Path to memory directory
    #[arg(short, long, default_value = "~/.openclaw/workspace/memory")]
    memory_dir: PathBuf,

    /// Number of days to review
    #[arg(short, long, default_value = "2")]
    days: usize,

    /// Path to MEMORY.md file
    #[arg(short, long, default_value = "~/.openclaw/workspace/MEMORY.md")]
    memory_file: PathBuf,

    /// Output report path
    #[arg(short, long, default_value = "/tmp/curator-report.json")]
    output_report: PathBuf,

    /// Confidence threshold (0.0-1.0)
    #[arg(short = 'c', long, default_value = "0.7")]
    confidence: f32,

    /// Max entries per run
    #[arg(short = 'm', long, default_value = "5")]
    max_entries: usize,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_max_level(tracing::Level::INFO)
        .init();

    let args = Args::parse();

    let config = CuratorConfig {
        memory_dir: args.memory_dir,
        days_to_review: args.days,
        memory_file: args.memory_file,
        max_daily_entries: args.max_entries,
        min_signal_confidence: args.confidence,
        prune_threshold_days: 90,
    };

    let mut curator = MemoryCurator::new(config)?;

    // Scan recent files
    let files = curator.scan_daily_files()?;
    println!("📋 Scanned {} files", files.len());

    // Extract signal
    let signals = curator.extract_signal(files)?;
    println!("✨ Found {} signal entries", signals.len());

    // Merge into memory
    curator.merge_into_memory(signals)?;
    println!("📝 Merged entries into MEMORY.md");

    // Prune stale entries
    let pruned = curator.prune_stale()?;
    println!("🧹 Pruned {} stale entries", pruned);

    // Generate report
    let report_json = curator.finalize_report()?;
    std::fs::write(&args.output_report, &report_json)?;
    println!("📊 Report written to {}", args.output_report.display());

    println!("\n{}", curator.report().summary());
    Ok(())
}
