use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct WalletAnalysis {
    pub authority: Pubkey,
    pub wallet: Pubkey,
    pub slot_start: u64,
    pub slot_end: u64,
    pub tx_count: u32,
    pub total_outflow: u64,
    pub total_inflow: u64,
    pub cluster_count: u16,
    pub tier: u8,
    pub last_updated: i64,
    pub _padding: [u8; 6],
}

impl WalletAnalysis {
    pub const LEN: usize = 32 + 32 + 8 + 8 + 4 + 8 + 8 + 2 + 1 + 8 + 6;
}

#[account]
#[derive(Default)]
pub struct ReceiverNode {
    pub wallet_analysis: Pubkey,
    pub receiver: Pubkey,
    pub tx_count: u32,
    pub total_amount: u64,
    pub first_seen: i64,
    pub last_seen: i64,
    pub category: u8,
    pub risk_score: u8,
    pub _padding: [u8; 6],
}

impl ReceiverNode {
    pub const LEN: usize = 32 + 32 + 4 + 8 + 8 + 8 + 1 + 1 + 6;
}

#[account]
#[derive(Default)]
pub struct TierConfig {
    pub authority: Pubkey,
    pub required_balance: [u64; 5],
    pub max_queries: [u32; 5],
    pub max_agents: [u16; 5],
    pub bump: u8,
    pub _padding: [u8; 7],
}

impl TierConfig {
    pub const LEN: usize = 32 + 40 + 20 + 10 + 1 + 7;
}
