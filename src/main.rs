use clawrtc::{MinerInfo, NodeClient};
use serde::Serialize;
use std::{env, process};

const DEFAULT_NODE: &str = "https://rustchain.org";

#[derive(Debug, PartialEq)]
struct CliOptions {
    node_url: String,
    args: Vec<String>,
}

#[derive(Debug, Serialize)]
struct StatusOutput {
    node_ok: bool,
    node_version: String,
    uptime_seconds: f64,
    wallet: Option<String>,
    balance_rtc: Option<f64>,
    mining_active: Option<bool>,
    active_miners: usize,
}

#[derive(Debug, Serialize)]
struct BalanceOutput {
    wallet: String,
    balance_rtc: f64,
}

#[derive(Debug, Serialize)]
struct MinerStatsOutput {
    active_miners: usize,
    miners: Vec<MinerInfo>,
}

fn main() {
    if let Err(err) = run(env::args().skip(1).collect()) {
        eprintln!("{err}");
        process::exit(1);
    }
}

fn run(raw_args: Vec<String>) -> Result<(), String> {
    let options = parse_global_options(raw_args)?;
    let (command, rest) = options
        .args
        .split_first()
        .ok_or_else(|| usage("missing command"))?;

    match command.as_str() {
        "status" => status(&options.node_url, rest),
        "wallet" => wallet(&options.node_url, rest),
        "miner" => miner(&options.node_url, rest),
        "-h" | "--help" | "help" => {
            print_usage();
            Ok(())
        }
        _ => Err(usage(&format!("unknown command: {command}"))),
    }
}

fn status(node_url: &str, args: &[String]) -> Result<(), String> {
    let json = has_flag(args, "--json");
    let wallet = option_value(args, "--wallet")?;
    reject_unknown_flags(args, &["--json", "--wallet"])?;

    let node = NodeClient::new(node_url);
    let health = node
        .health()
        .map_err(|err| format!("status failed: {err}"))?;
    let miners = node
        .miners()
        .map_err(|err| format!("miner stats failed: {err}"))?;
    let balance = match wallet.as_deref() {
        Some(address) => Some(
            node.balance(address)
                .map_err(|err| format!("balance failed for {address}: {err}"))?,
        ),
        None => None,
    };
    let mining_active = wallet.as_deref().map(|address| {
        miners
            .iter()
            .any(|miner| miner.miner == address || miner.miner_id == address)
    });

    let output = StatusOutput {
        node_ok: health.ok,
        node_version: health.version,
        uptime_seconds: health.uptime_s,
        wallet,
        balance_rtc: balance,
        mining_active,
        active_miners: miners.len(),
    };

    if json {
        print_json(&output)
    } else {
        println!("Node OK: {}", output.node_ok);
        println!("Node version: {}", output.node_version);
        println!("Uptime: {}", format_duration(output.uptime_seconds));
        println!("Active miners: {}", output.active_miners);
        if let Some(wallet) = &output.wallet {
            println!("Wallet: {wallet}");
        }
        if let Some(balance) = output.balance_rtc {
            println!("Balance: {balance} RTC");
        }
        if let Some(active) = output.mining_active {
            println!("Mining active: {active}");
        }
        Ok(())
    }
}

fn wallet(node_url: &str, args: &[String]) -> Result<(), String> {
    let (subcommand, rest) = args
        .split_first()
        .ok_or_else(|| usage("missing wallet subcommand"))?;
    match subcommand.as_str() {
        "balance" => wallet_balance(node_url, rest),
        _ => Err(usage(&format!("unknown wallet subcommand: {subcommand}"))),
    }
}

fn wallet_balance(node_url: &str, args: &[String]) -> Result<(), String> {
    let json = has_flag(args, "--json");
    reject_unknown_flags(args, &["--json"])?;

    let positional = positional_args(args);
    let wallet = positional
        .first()
        .ok_or_else(|| usage("wallet balance requires a wallet address"))?;
    if positional.len() > 1 {
        return Err(usage("wallet balance accepts only one wallet address"));
    }

    let node = NodeClient::new(node_url);
    let balance = node
        .balance(wallet)
        .map_err(|err| format!("balance failed for {wallet}: {err}"))?;
    let output = BalanceOutput {
        wallet: wallet.to_string(),
        balance_rtc: balance,
    };

    if json {
        print_json(&output)
    } else {
        println!("Wallet: {}", output.wallet);
        println!("Balance: {} RTC", output.balance_rtc);
        Ok(())
    }
}

fn miner(node_url: &str, args: &[String]) -> Result<(), String> {
    let (subcommand, rest) = args
        .split_first()
        .ok_or_else(|| usage("missing miner subcommand"))?;
    match subcommand.as_str() {
        "stats" => miner_stats(node_url, rest),
        _ => Err(usage(&format!("unknown miner subcommand: {subcommand}"))),
    }
}

fn miner_stats(node_url: &str, args: &[String]) -> Result<(), String> {
    let json = has_flag(args, "--json");
    reject_unknown_flags(args, &["--json"])?;
    if !positional_args(args).is_empty() {
        return Err(usage("miner stats does not accept positional arguments"));
    }

    let node = NodeClient::new(node_url);
    let miners = node
        .miners()
        .map_err(|err| format!("miner stats failed: {err}"))?;
    let output = MinerStatsOutput {
        active_miners: miners.len(),
        miners,
    };

    if json {
        print_json(&output)
    } else {
        println!("Active miners: {}", output.active_miners);
        for miner in output.miners.iter().take(20) {
            let id = if miner.miner_id.is_empty() {
                &miner.miner
            } else {
                &miner.miner_id
            };
            println!(
                "- {} {} {} last_seen={}",
                id, miner.device_family, miner.device_arch, miner.last_seen
            );
        }
        if output.active_miners > 20 {
            println!("... {} more", output.active_miners - 20);
        }
        Ok(())
    }
}

fn parse_global_options(raw_args: Vec<String>) -> Result<CliOptions, String> {
    let mut node_url = DEFAULT_NODE.to_string();
    let mut args = Vec::new();
    let mut iter = raw_args.into_iter();

    while let Some(arg) = iter.next() {
        if arg == "--node" {
            node_url = iter.next().ok_or_else(|| usage("--node requires a URL"))?;
        } else if let Some(value) = arg.strip_prefix("--node=") {
            node_url = value.to_string();
        } else {
            args.push(arg);
            args.extend(iter);
            break;
        }
    }

    Ok(CliOptions { node_url, args })
}

fn has_flag(args: &[String], flag: &str) -> bool {
    args.iter().any(|arg| arg == flag)
}

fn option_value(args: &[String], flag: &str) -> Result<Option<String>, String> {
    let mut value = None;
    let mut iter = args.iter().enumerate();
    while let Some((index, arg)) = iter.next() {
        if arg == flag {
            let (_, next) = iter
                .next()
                .ok_or_else(|| usage(&format!("{flag} requires a value")))?;
            value = Some(next.to_string());
        } else if let Some(next) = arg.strip_prefix(&format!("{flag}=")) {
            value = Some(next.to_string());
        } else if index > 0 {
            continue;
        }
    }
    Ok(value)
}

fn reject_unknown_flags(args: &[String], allowed: &[&str]) -> Result<(), String> {
    let mut skip_next = false;
    for arg in args {
        if skip_next {
            skip_next = false;
            continue;
        }
        if !arg.starts_with("--") {
            continue;
        }

        let flag = arg.split_once('=').map_or(arg.as_str(), |(flag, _)| flag);
        if !allowed.contains(&flag) {
            return Err(usage(&format!("unknown flag: {flag}")));
        }
        if flag == "--wallet" && !arg.contains('=') {
            skip_next = true;
        }
    }
    Ok(())
}

fn positional_args(args: &[String]) -> Vec<&str> {
    let mut values = Vec::new();
    let mut skip_next = false;
    for arg in args {
        if skip_next {
            skip_next = false;
            continue;
        }
        if arg == "--wallet" {
            skip_next = true;
            continue;
        }
        if arg.starts_with("--") {
            continue;
        }
        values.push(arg.as_str());
    }
    values
}

fn print_json<T: Serialize>(value: &T) -> Result<(), String> {
    let json = serde_json::to_string_pretty(value)
        .map_err(|err| format!("failed to encode JSON: {err}"))?;
    println!("{json}");
    Ok(())
}

fn format_duration(seconds: f64) -> String {
    let seconds = seconds.max(0.0) as u64;
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let seconds = seconds % 60;

    if hours > 0 {
        format!("{hours}h {minutes}m {seconds}s")
    } else if minutes > 0 {
        format!("{minutes}m {seconds}s")
    } else {
        format!("{seconds}s")
    }
}

fn usage(message: &str) -> String {
    format!("{message}\n\n{}", usage_text())
}

fn print_usage() {
    println!("{}", usage_text());
}

fn usage_text() -> &'static str {
    "Usage:
  clawrtc [--node URL] status [--json] [--wallet WALLET]
  clawrtc [--node URL] wallet balance WALLET [--json]
  clawrtc [--node URL] miner stats [--json]"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_global_node_option() {
        let options = parse_global_options(vec![
            "--node".into(),
            "https://example.test".into(),
            "status".into(),
            "--json".into(),
        ])
        .unwrap();

        assert_eq!(
            options,
            CliOptions {
                node_url: "https://example.test".into(),
                args: vec!["status".into(), "--json".into()],
            }
        );
    }

    #[test]
    fn parses_wallet_option_value() {
        let args = vec!["--json".into(), "--wallet=RTCabc".into()];
        assert_eq!(
            option_value(&args, "--wallet").unwrap(),
            Some("RTCabc".into())
        );
    }

    #[test]
    fn formats_duration() {
        assert_eq!(format_duration(59.0), "59s");
        assert_eq!(format_duration(61.0), "1m 1s");
        assert_eq!(format_duration(3661.0), "1h 1m 1s");
    }
}
