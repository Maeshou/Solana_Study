// 10. 利益分配＋配当記録（Clockなし）
use anchor_lang::prelude::*;
declare_id!("DIVIDZZZZYYYYXXXXWWWWVVVVUUUUTTTT");

#[program]
pub mod misinit_dividend_no_clock {
    use super::*;

    pub fn init_dividend(ctx: Context<InitDividend>, total: u128) -> Result<()> {
        let dv = &mut ctx.accounts.dividend;
        dv.total = total;
        dv.distributed = 0;
        Ok(())
    }

    pub fn distribute(ctx: Context<InitDividend>, share: u128, to: Pubkey) -> Result<()> {
        let dv = &mut ctx.accounts.dividend;
        require!(share <= dv.total - dv.distributed, ErrorCode6::OverAlloc);
        dv.distributed += share;
        let log = &mut ctx.accounts.dividend_log;
        if log.entries.len() >= 10 { log.entries.remove(0); }
        log.entries.push((to, share));
        Ok(())
    }
}

#[derive(Accounts)]

pub struct InitDividend<'info> {
    #[account(init_if_needed, payer = payer, space = 8 + 16 + 16)] pub dividend: Account<'info, DividendData>,
    #[account(mut)] pub dividend_log: Account<'info, DividendLog>,
    #[account(mut)] pub payer: Signer<'info>, pub system_program: Program<'info, System>,
}

#[account]
pub struct DividendData { pub total:u128, pub distributed:u128 }
#[account]
pub struct DividendLog { pub entries: Vec<(Pubkey,u128)> }

#[error_code]
pub enum ErrorCode6 { #[msg("配分上限を超えています。")] OverAlloc }
