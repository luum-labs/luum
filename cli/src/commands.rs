use clap::{Args, Subcommand};

#[derive(Subcommand)]
pub enum Commands {
    Analyze(AnalyzeArgs),
    Export(ExportArgs),
}

#[derive(Args)]
pub struct AnalyzeArgs {
    #[arg(short, long, help = "Solana wallet address to analyze")]
    pub address: String,

    #[arg(short, long, default_value = "7", help = "Number of days to look back")]
    pub days: u32,

    #[arg(
        short,
        long,
        default_value = "100",
        help = "Minimum transfer amount in lamports"
    )]
    pub min_amount: u64,

    #[arg(long, default_value = "false", help = "Include inbound transfers")]
    pub include_inbound: bool,
}

#[derive(Args)]
pub struct ExportArgs {
    #[arg(short, long, help = "Solana wallet address")]
    pub address: String,

    #[arg(
        short,
        long,
        default_value = "json",
        help = "Output format: json or csv"
    )]
    pub format: String,

    #[arg(short, long, help = "Output file path")]
    pub output: Option<String>,
}
