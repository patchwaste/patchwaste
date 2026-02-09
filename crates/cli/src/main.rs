use std::path::{Path, PathBuf};

use anyhow::Context;
use clap::{Parser, Subcommand};

use patchwaste_core::{analyze_dir, AnalyzeOptions};

#[derive(Parser, Debug)]
#[command(
    name = "patchwaste",
    version,
    about = "SteamPipe patch efficiency gate (estimated)"
)]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Analyze {
        #[arg(long)]
        input: PathBuf,

        #[arg(long)]
        baseline: Option<PathBuf>,

        #[arg(long)]
        budget_ratio: Option<f64>,

        #[arg(long)]
        strict: bool,

        #[arg(long, default_value = "patchwaste-out")]
        out: PathBuf,
    },
}

fn main() -> std::process::ExitCode {
    let cli = Cli::parse();

    let res = match cli.cmd {
        Commands::Analyze {
            input,
            baseline,
            budget_ratio,
            strict,
            out,
        } => run_analyze(&input, baseline.as_deref(), budget_ratio, strict, &out),
    };

    match res {
        Ok(code) => code,
        Err(e) => {
            eprintln!("error: {:#}", e);
            std::process::ExitCode::from(1)
        }
    }
}

fn run_analyze(
    input: &Path,
    baseline: Option<&Path>,
    budget_ratio: Option<f64>,
    strict: bool,
    out: &Path,
) -> anyhow::Result<std::process::ExitCode> {
    let opts = AnalyzeOptions {
        strict,
        budget_ratio,
        baseline_path: baseline.map(|p| p.to_path_buf()),
        ..AnalyzeOptions::default()
    };

    let mut report = analyze_dir(input, opts)?;
    report.inputs.input_path = input.display().to_string();

    std::fs::create_dir_all(out).with_context(|| format!("create out dir {}", out.display()))?;

    let json_path = out.join("report.json");
    let md_path = out.join("report.md");

    let json = serde_json::to_vec_pretty(&report).context("serialize report json")?;
    std::fs::write(&json_path, json).with_context(|| format!("write {}", json_path.display()))?;

    let md = report.to_markdown();
    std::fs::write(&md_path, md).with_context(|| format!("write {}", md_path.display()))?;

    println!(
        "new_bytes={} changed_content_bytes={} waste_ratio={:.3}",
        report.metrics.new_bytes, report.metrics.changed_content_bytes, report.metrics.waste_ratio
    );

    let exit = match &report.budget {
        Some(b) if !b.pass => std::process::ExitCode::from(2),
        _ => std::process::ExitCode::from(0),
    };

    Ok(exit)
}
