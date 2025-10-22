use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
#[instruction(wallet: Pubkey, slot_start: u64, slot_end: u64)]
pub struct CreateAnalysis<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + WalletAnalysis::LEN,
        seeds = [b"analysis", authority.key().as_ref(), wallet.as_ref()],
        bump,
    )]
    pub analysis: Account<'info, WalletAnalysis>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateAnalysis<'info> {
    #[account(
        mut,
        has_one = authority,
        seeds = [b"analysis", authority.key().as_ref(), analysis.wallet.as_ref()],
        bump,
    )]
    pub analysis: Account<'info, WalletAnalysis>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(receiver: Pubkey)]
pub struct AddReceiver<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + ReceiverNode::LEN,
        seeds = [b"receiver", analysis.key().as_ref(), receiver.as_ref()],
        bump,
    )]
    pub node: Account<'info, ReceiverNode>,
    #[account(has_one = authority)]
    pub analysis: Account<'info, WalletAnalysis>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitTierConfig<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + TierConfig::LEN,
        seeds = [b"tier_config"],
        bump,
    )]
    pub config: Account<'info, TierConfig>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}
