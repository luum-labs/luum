use clap::{Parser, Subcommand};

mod commands;
use commands::{AnalyzeArgs, Commands, ExportArgs};

#[derive(Parser)]
#[command(name = "luum-cli")]
#[command(about = "High-frequency x402 micro-payment analysis engine for Solana AI agents")]
#[command(version = "0.4.2")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn run_analyze(args: &AnalyzeArgs) {
    println!("Analyzing wallet: {}", args.address);
    println!(
        "Looking back {} days, min amount: {}",
        args.days, args.min_amount
    );

    let sample_data: Vec<(String, u64, u32)> = vec![
        ("Api1111111111111111111111111111111".into(), 50_000, 1200),
        ("Orac222222222222222222222222222222".into(), 12_000, 340),
        ("Comp333333333333333333333333333333".into(), 8_500, 890),
    ];

    let engine = luum_math::ClusterEngine::new(args.min_amount, 10);
    let clusters = engine.cluster(&sample_data);

    for cluster in &clusters {
        println!(
            "  [{:?}] {} -- {} TXs, {} total, risk: {:.0}",
            cluster.category,
            &cluster.address[..8],
            cluster.tx_count,
            cluster.total_amount,
            cluster.risk_score,
        );
    }

    let flows: Vec<(String, u64, String)> = clusters
        .iter()
        .map(|c| {
            (
                c.address.clone(),
                c.total_amount,
                format!("{:?}", c.category),
            )
        })
        .collect();

    let graph = luum_math::SankeyBuilder::new(&args.address[..8])
        .with_min_value(1000)
        .build(&flows);

    println!(
        "Sankey: {} nodes, {} links, total flow: {}",
        graph.nodes.len(),
        graph.links.len(),
        graph.total_flow,
    );
}

fn run_export(args: &ExportArgs) {
    let dest = args.output.as_deref().unwrap_or("stdout");
    println!(
        "Exporting {} data for {} to {}",
        args.format, args.address, dest
    );
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Analyze(args) => run_analyze(args),
        Commands::Export(args) => run_export(args),
    }
}
