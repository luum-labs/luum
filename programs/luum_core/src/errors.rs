use anchor_lang::prelude::*;

#[error_code]
pub enum LuumError {
    #[msg("Transaction window exceeds maximum allowed range")]
    WindowOverflow,
    #[msg("Invalid wallet address format")]
    InvalidAddress,
    #[msg("Delegation authority not found for this account")]
    DelegationNotFound,
    #[msg("Insufficient token balance for requested tier")]
    InsufficientBalance,
    #[msg("Analysis slot range must be positive")]
    InvalidSlotRange,
    #[msg("Receiver cluster exceeds capacity limit")]
    ClusterOverflow,
    #[msg("Revoke instruction failed on target program")]
    RevokeFailed,
    #[msg("Rate limit exceeded for current tier")]
    RateLimitExceeded,
}
