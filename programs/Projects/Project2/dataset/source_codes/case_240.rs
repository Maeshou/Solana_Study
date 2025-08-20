use anchor_lang::prelude::*;
use std::collections::BTreeMap;

declare_id!("RefundHist099999999999999999999999999999999");

#[program]
pub mod refund_history {
    use super::*;

    pub fn request_refund(ctx: Context<Refund>, offer_id: u64) -> Result<()> {
        let rh = &mut ctx.accounts.history;
        rh.requests.push(offer_id);
        if rh.requests.len() > 100 {
            rh.requests.remove(0);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Refund<'info> {
    #[account(mut)]
    pub history: Account<'info, RefundData>,
}

#[account]
pub struct RefundData {
    pub requests: Vec<u64>,
}
