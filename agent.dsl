// Memory Curator Agent
// Maintains long-term memory by curating daily logs into actionable insights
// Language: Symbiont DSL v0.6+

agent "memory-curator" {
  version = "0.1.0"
  description = "Curate daily memory logs into MEMORY.md, identify signal, prune stale entries"
  author = "Golum <golum@openclaw.local>"
  
  // Input schema
  input {
    memory_dir: string = "/home/jascha/.openclaw/workspace/memory"
    days_to_review: int = 2
    memory_file: string = "/home/jascha/.openclaw/workspace/MEMORY.md"
    max_daily_entries: int = 5
    min_signal_confidence: float = 0.7
    prune_threshold_days: int = 90
  }

  // Output schema
  output {
    status: string
    entries_added: int
    entries_removed: int
    report_path: string
    timestamp: string
  }

  // Permissions: read daily files, write MEMORY.md, create reports
  permissions {
    files {
      read = [
        "/home/jascha/.openclaw/workspace/memory/*.md"
      ]
      write = [
        "/home/jascha/.openclaw/workspace/MEMORY.md",
        "/tmp/curator-*.json"
      ]
    }
    
    network = none
    secrets = none
  }

  // Sandbox policy: Tier1 (Docker, full isolation)
  sandbox {
    tier = 1
    timeout_seconds = 300
    memory_mb = 512
    cpu_limit = 1
  }

  // Audit trail: sign all file mutations
  audit {
    crypto = "ed25519"
    log_mutations = true
    log_level = "info"
  }

  // Initialization: validate directories exist
  init {
    require_dir(memory_dir)
    require_file(memory_file)
  }

  // Main logic: scan, identify, update
  task "curator" {
    steps = [
      "scan_recent_logs",
      "identify_signal",
      "update_memory_file",
      "prune_stale_entries",
      "generate_report"
    ]
  }

  // Step definitions
  step "scan_recent_logs" {
    script = "src/curator.rs::scan_daily_files"
    timeout = 30
  }

  step "identify_signal" {
    script = "src/curator.rs::extract_signal"
    timeout = 60
    depends_on = ["scan_recent_logs"]
  }

  step "update_memory_file" {
    script = "src/curator.rs::merge_into_memory"
    timeout = 30
    depends_on = ["identify_signal"]
  }

  step "prune_stale_entries" {
    script = "src/curator.rs::prune_old_entries"
    timeout = 30
    depends_on = ["update_memory_file"]
  }

  step "generate_report" {
    script = "src/curator.rs::write_report"
    timeout = 10
    depends_on = ["prune_stale_entries"]
  }

  // Error handling
  error_handling {
    on_file_not_found = "skip_and_log"
    on_parse_error = "skip_and_log"
    on_timeout = "abort_and_report"
    on_permission_denied = "abort_and_report"
  }

  // Recovery: never corrupt source files
  rollback {
    strategy = "snapshot"
    on_failure = "restore_previous_memory_md"
  }
}
