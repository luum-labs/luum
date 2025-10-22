use anchor_lang::prelude::*;

pub mod contexts;
pub mod errors;
pub mod events;
pub mod state;
pub mod utils;

use contexts::*;
use errors::LuumError;
use events::*;

declare_id!("LUMx4VrS8NqZgP7tGbHP4bXWk2HqTdGe6NxVCrnAK5v");

#[program]
pub mod luum_core {
    use super::*;

    pub fn create_analysis(
        ctx: Context<CreateAnalysis>,
        wallet: Pubkey,
        slot_start: u64,
        slot_end: u64,
    ) -> Result<()> {
        let range = utils::validate_slot_range(slot_start, slot_end)?;
        let analysis = &mut ctx.accounts.analysis;
        analysis.authority = ctx.accounts.authority.key();
        analysis.wallet = wallet;
        analysis.slot_start = slot_start;
        analysis.slot_end = slot_end;
        analysis.tx_count = 0;
        analysis.total_outflow = 0;
        analysis.total_inflow = 0;
        analysis.cluster_count = 0;
        analysis.last_updated = Clock::get()?.unix_timestamp;

        emit!(AnalysisCreated {
            wallet,
            slot_start,
            slot_end,
            tx_count: 0,
            timestamp: analysis.last_updated,
        });

        msg!("Analysis created for wallet {} over {} slots", wallet, range);
        Ok(())
    }

    pub fn update_analysis(
        ctx: Context<UpdateAnalysis>,
        tx_count: u32,
        outflow: u64,
        inflow: u64,
        cluster_count: u16,
    ) -> Result<()> {
        let analysis = &mut ctx.accounts.analysis;
        analysis.tx_count = tx_count;
        analysis.total_outflow = outflow;
        analysis.total_inflow = inflow;
        analysis.cluster_count = cluster_count;
        analysis.last_updated = Clock::get()?.unix_timestamp;

        emit!(ClusterUpdated {
            wallet: analysis.wallet,
            cluster_count,
            total_flow: outflow.saturating_add(inflow),
            timestamp: analysis.last_updated,
        });

        Ok(())
    }

    pub fn add_receiver(
        ctx: Context<AddReceiver>,
        receiver: Pubkey,
        tx_count: u32,
        total_amount: u64,
        category: u8,
    ) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;
        let node = &mut ctx.accounts.node;
        node.wallet_analysis = ctx.accounts.analysis.key();
        node.receiver = receiver;
        node.tx_count = tx_count;
        node.total_amount = total_amount;
        node.first_seen = now;
        node.last_seen = now;
        node.category = category;
        node.risk_score = utils::calculate_risk_score(tx_count, total_amount, 1);
        Ok(())
    }

    pub fn init_tier_config(
        ctx: Context<InitTierConfig>,
        balances: [u64; 5],
        max_queries: [u32; 5],
        max_agents: [u16; 5],
    ) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.authority = ctx.accounts.authority.key();
        config.required_balance = balances;
        config.max_queries = max_queries;
        config.max_agents = max_agents;
        Ok(())
    }
}
