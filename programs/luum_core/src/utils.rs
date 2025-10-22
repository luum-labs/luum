use anchor_lang::prelude::*;

pub fn calculate_risk_score(tx_count: u32, total_amount: u64, days_active: u64) -> u8 {
    if days_active == 0 {
        return 0;
    }
    let daily_avg = total_amount.checked_div(days_active).unwrap_or(0);
    let frequency = tx_count as u64;

    let freq_score = if frequency > 500 { 40 } else { (frequency * 40 / 500) as u8 };
    let amount_score = if daily_avg > 1_000_000 { 40 } else { (daily_avg * 40 / 1_000_000) as u8 };
    let consistency = if tx_count > 100 && days_active > 7 { 20 } else { 10 };

    freq_score.saturating_add(amount_score).saturating_add(consistency).min(100)
}

pub fn determine_tier(balance: u64, total_supply: u64) -> u8 {
    if total_supply == 0 {
        return 0;
    }
    let ratio_bps = balance.checked_mul(10_000).unwrap_or(0) / total_supply;

    if ratio_bps >= 10 { 4 }       // 0.1% = Whale
    else if ratio_bps >= 5 { 3 }   // 0.05% = Elite
    else if ratio_bps >= 1 { 2 }   // 0.01% = Pro
    else if ratio_bps > 0 { 1 }    // >0 = Basic
    else { 0 }                      // Free
}

pub fn validate_slot_range(start: u64, end: u64) -> Result<u64> {
    require!(end > start, crate::errors::LuumError::InvalidSlotRange);
    let range = end - start;
    require!(range <= 432_000 * 30, crate::errors::LuumError::WindowOverflow);
    Ok(range)
}
