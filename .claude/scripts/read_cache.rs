#!/usr/bin/env rust-script

//! Session cache reader for design-source methodology projects.
//!
//! This script reads and displays the session cache to help
//! restore context between AI assistant sessions.
//!
//! ## Usage
//!
//! ```bash
//! rust-script .claude/scripts/read_cache.rs
//! ```

use std::fs;
use std::path::Path;

fn main() {
    println!("🔍 Loading previous conversation context...");
    println!();

    let cache_path = ".claude/cache/session.json";

    if !Path::new(cache_path).exists() {
        println!("❌ No previous conversation found");
        println!("💡 Tip: Run /save-session-cache to create a cache for this project");
        return;
    }

    match fs::read_to_string(cache_path) {
        Ok(content) => {
            println!("✅ Context loaded successfully!");
            println!();

            // Parse and display cache content
            if let Ok(cache) = serde_json::from_str::<serde_json::Value>(&content) {
                display_cache(&cache);
            } else {
                println!("⚠️  Cache file exists but couldn't be parsed");
            }
        }
        Err(e) => {
            println!("❌ Failed to read cache: {}", e);
        }
    }
}

fn display_cache(cache: &serde_json::Value) {
    // Project info
    if let Some(project) = cache.get("project") {
        println!("📊 Project Information:");
        if let Some(name) = project.get("name").and_then(|v| v.as_str()) {
            println!("  Name: {}", name);
        }
        if let Some(version) = project.get("version").and_then(|v| v.as_str()) {
            println!("  Version: {}", version);
        }
        println!();
    }

    // Session info
    if let Some(session) = cache.get("session") {
        println!("📈 Session Information:");
        if let Some(count) = session.get("count").and_then(|v| v.as_u64()) {
            println!("  Session: #{}", count);
        }
        if let Some(last) = session.get("last_timestamp").and_then(|v| v.as_str()) {
            println!("  Last Session: {}", last);
        }
        println!();
    }

    // Current phase
    if let Some(phase) = cache.get("current_phase") {
        println!("📍 Current Phase:");
        if let Some(name) = phase.get("name").and_then(|v| v.as_str()) {
            println!("  Phase: {}", name);
        }
        if let Some(status) = phase.get("status").and_then(|v| v.as_str()) {
            println!("  Status: {}", status);
        }
        println!();
    }

    // Pending tasks
    if let Some(tasks) = cache.get("pending_tasks").and_then(|v| v.as_array()) {
        if !tasks.is_empty() {
            println!("📌 Pending Tasks:");
            for (i, task) in tasks.iter().take(5).enumerate() {
                if let Some(desc) = task.as_str() {
                    println!("  {}. {}", i + 1, desc);
                }
            }
            if tasks.len() > 5 {
                println!("  ... and {} more", tasks.len() - 5);
            }
            println!();
        }
    }

    // Blockers
    if let Some(blockers) = cache.get("blockers").and_then(|v| v.as_array()) {
        if !blockers.is_empty() {
            println!("🚧 Active Blockers:");
            for blocker in blockers {
                if let Some(desc) = blocker.as_str() {
                    println!("  ⚠️  {}", desc);
                }
            }
            println!();
        }
    }

    println!("🚀 Ready to continue where we left off!");
}
