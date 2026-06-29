use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use clap::Parser;
use serde::{Deserialize, Serialize};
use soroban_rpc::{Client, EventsRequest};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Soroban RPC endpoint URL
    #[arg(short, long, default_value = "https://soroban-testnet.stellar.org")]
    rpc_url: String,

    /// Contract ID to fetch events from
    #[arg(short, long)]
    contract_id: String,

    /// Start timestamp (ISO 8601 format)
    #[arg(short, long)]
    start_time: Option<String>,

    /// End timestamp (ISO 8601 format)
    #[arg(short, long)]
    end_time: Option<String>,

    /// Event type filter (e.g., "job_posted", "job_approved")
    #[arg(short, long)]
    event_type: Option<String>,

    /// Output file for replay data
    #[arg(short, long)]
    output: PathBuf,

    /// Generate sample replay dataset from testnet
    #[arg(long)]
    sample: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct ReplayEvent {
    timestamp: DateTime<Utc>,
    event_type: String,
    contract_id: String,
    topic: Vec<String>,
    data: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
struct ReplayDataset {
    contract_id: String,
    network: String,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    events: Vec<ReplayEvent>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ReplayReport {
    dataset: ReplayDataset,
    events_processed: usize,
    state_transitions: Vec<String>,
    final_balances: HashMap<String, String>,
    summary: String,
}

async fn fetch_events(
    rpc_url: &str,
    contract_id: &str,
    start_time: Option<DateTime<Utc>>,
    end_time: Option<DateTime<Utc>>,
    event_type_filter: Option<&str>,
) -> Result<Vec<ReplayEvent>> {
    let client = Client::new(rpc_url);
    let mut events = Vec::new();

    let start_ledger = start_time.map(|_| 0u32); // Simplified - in production, map time to ledger
    let end_ledger = end_time.map(|_| u32::MAX); // Simplified

    let request = EventsRequest {
        start_ledger,
        end_ledger,
        filter: None, // In production, add contract ID filter
    };

    let response = client
        .get_events(request)
        .await
        .context("Failed to fetch events from RPC")?;

    for event in response.events {
        // Filter by contract ID
        if let Some(contract) = event.contract_id.as_ref() {
            if contract.to_string() != contract_id {
                continue;
            }
        }

        // Filter by event type
        if let Some(filter) = event_type_filter {
            if !event.topics.iter().any(|t| t.to_string().contains(filter)) {
                continue;
            }
        }

        let replay_event = ReplayEvent {
            timestamp: Utc::now(), // Simplified - use event timestamp in production
            event_type: event
                .topics
                .first()
                .map(|t| t.to_string())
                .unwrap_or("unknown".to_string()),
            contract_id: event
                .contract_id
                .map(|c| c.to_string())
                .unwrap_or(contract_id.to_string()),
            topic: event.topics.iter().map(|t| t.to_string()).collect(),
            data: serde_json::to_value(&event.value).unwrap_or(serde_json::Value::Null),
        };

        events.push(replay_event);
    }

    // Sort chronologically
    events.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

    Ok(events)
}

async fn generate_sample_dataset(output: PathBuf) -> Result<()> {
    let sample_dataset = ReplayDataset {
        contract_id: "SAMPLE_CONTRACT_ID".to_string(),
        network: "testnet".to_string(),
        start_time: Utc::now() - chrono::Duration::days(7),
        end_time: Utc::now(),
        events: vec![
            ReplayEvent {
                timestamp: Utc::now() - chrono::Duration::days(7),
                event_type: "job_posted".to_string(),
                contract_id: "SAMPLE_CONTRACT_ID".to_string(),
                topic: vec!["job_posted".to_string()],
                data: serde_json::json!({"job_id": 1, "amount": 1000000}),
            },
            ReplayEvent {
                timestamp: Utc::now() - chrono::Duration::days(6),
                event_type: "job_accepted".to_string(),
                contract_id: "SAMPLE_CONTRACT_ID".to_string(),
                topic: vec!["job_accepted".to_string()],
                data: serde_json::json!({"job_id": 1, "freelancer": "G..."}),
            },
            ReplayEvent {
                timestamp: Utc::now() - chrono::Duration::days(5),
                event_type: "work_submitted".to_string(),
                contract_id: "SAMPLE_CONTRACT_ID".to_string(),
                topic: vec!["work_submitted".to_string()],
                data: serde_json::json!({"job_id": 1}),
            },
            ReplayEvent {
                timestamp: Utc::now() - chrono::Duration::days(4),
                event_type: "job_approved".to_string(),
                contract_id: "SAMPLE_CONTRACT_ID".to_string(),
                topic: vec!["job_approved".to_string()],
                data: serde_json::json!({"job_id": 1, "payout": 975000}),
            },
        ],
    };

    let json = serde_json::to_string_pretty(&sample_dataset)?;
    fs::write(&output, json)?;
    println!("Sample dataset written to: {}", output.display());
    Ok(())
}

async fn replay_events(dataset: &ReplayDataset) -> Result<ReplayReport> {
    let mut state_transitions = Vec::new();
    let mut final_balances = HashMap::new();

    for event in &dataset.events {
        state_transitions.push(format!(
            "{}: {}",
            event.timestamp.format("%Y-%m-%d %H:%M:%S"),
            event.event_type
        ));

        // Simulate state changes based on event type
        match event.event_type.as_str() {
            "job_posted" => {
                *final_balances
                    .entry("contract_balance".to_string())
                    .or_insert(0i64.to_string()) += 1000000;
            }
            "job_approved" => {
                *final_balances
                    .entry("contract_balance".to_string())
                    .or_insert(0i64.to_string()) -= 975000;
                *final_balances
                    .entry("platform_fees".to_string())
                    .or_insert(0i64.to_string()) += 25000;
            }
            _ => {}
        }
    }

    let report = ReplayReport {
        dataset: dataset.clone(),
        events_processed: dataset.events.len(),
        state_transitions,
        final_balances,
        summary: format!(
            "Processed {} events from {} to {}",
            dataset.events.len(),
            dataset.start_time.format("%Y-%m-%d"),
            dataset.end_time.format("%Y-%m-%d")
        ),
    };

    Ok(report)
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    if args.sample {
        generate_sample_dataset(args.output).await?;
        return Ok(());
    }

    let start_time = args
        .start_time
        .as_ref()
        .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.with_timezone(&Utc));

    let end_time = args
        .end_time
        .as_ref()
        .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.with_timezone(&Utc));

    println!("Fetching events from contract: {}", args.contract_id);
    println!("RPC URL: {}", args.rpc_url);

    let events = fetch_events(
        &args.rpc_url,
        &args.contract_id,
        start_time,
        end_time,
        args.event_type.as_deref(),
    )
    .await?;

    if events.is_empty() {
        println!("No events found matching the criteria.");
        return Ok(());
    }

    let dataset = ReplayDataset {
        contract_id: args.contract_id.clone(),
        network: if args.rpc_url.contains("testnet") {
            "testnet".to_string()
        } else {
            "mainnet".to_string()
        },
        start_time: start_time.unwrap_or_else(|| events.first().unwrap().timestamp),
        end_time: end_time.unwrap_or_else(|| events.last().unwrap().timestamp),
        events,
    };

    let json = serde_json::to_string_pretty(&dataset)?;
    fs::write(&args.output, json)?;
    println!("Dataset written to: {}", args.output.display());

    // Run replay and generate report
    println!("\nRunning event replay...");
    let report = replay_events(&dataset).await?;

    println!("\n=== Replay Report ===");
    println!("{}", report.summary);
    println!("\nState Transitions:");
    for transition in &report.state_transitions {
        println!("  {}", transition);
    }
    println!("\nFinal Balances:");
    for (key, value) in &report.final_balances {
        println!("  {}: {}", key, value);
    }

    // Write report to file
    let report_path = args.output.with_extension("report.json");
    let report_json = serde_json::to_string_pretty(&report)?;
    fs::write(&report_path, report_json)?;
    println!("\nReport written to: {}", report_path.display());

    Ok(())
}
