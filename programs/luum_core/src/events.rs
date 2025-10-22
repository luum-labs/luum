use anchor_lang::prelude::*;

#[event]
pub struct AnalysisCreated {
    pub wallet: Pubkey,
    pub slot_start: u64,
    pub slot_end: u64,
    pub tx_count: u32,
    pub timestamp: i64,
}

#[event]
pub struct DelegationRevoked {
    pub wallet: Pubkey,
    pub delegate: Pubkey,
    pub amount_saved: u64,
    pub timestamp: i64,
}

#[event]
pub struct ClusterUpdated {
    pub wallet: Pubkey,
    pub cluster_count: u16,
    pub total_flow: u64,
    pub timestamp: i64,
}
