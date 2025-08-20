// 4) scheduled_window_program — 時間帯でIDを切り替え
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use anchor_lang::solana_program::{program::invoke, sysvar::clock::Clock};
use anchor_spl::token::spl_token;

declare_id!("Sch3dul3dWind0w4444444444444444444444444");

#[program]
pub mod scheduled_window_program {
    use super::*;

    pub fn setup(ctx: Context<Setup>, day_id: Pubkey, night_id: Pubkey) -> Result<()> {
        let s = &mut ctx.accounts.state;
        s.admin = ctx.accounts.admin.key();
        s.day = day_id;
        s.night = night_id;
        s.counter = 0;
        Ok(())
    }

    pub fn pay(ctx: Context<Pay>, base: u64) -> Result<()> {
        let s = &mut ctx.accounts.state;
        require!(s.admin == ctx.accounts.admin.key(), Errs::Admin);

        let clock = &ctx.accounts.clock;
        let mut chosen = s.day;
        // 単純に slot の偶奇で切替（例示）
        if clock.slot % 2 == 1 {
            chosen = s.night;
        } else {
            s.counter = s.counter.saturating_add(1);
        }

        let mut amt = base;
        let mut k = 0u8;
        while k < 5 {
            amt = amt.saturating_add((k as u64) + ((s.counter % 7) as u64));
            k = k.saturating_add(1);
        }

        let ix = spl_token::instruction::transfer(
            chosen, // ← 時間帯で選ばれたID
            ctx.accounts.treasury.key(),
            ctx.accounts.recipient_ata.key(),
            ctx.accounts.admin.key(),
            &[],
            amt,
        )?;
        invoke(&ix, &[
            ctx.accounts.treasury.to_account_info(),
            ctx.accounts.recipient_ata.to_account_info(),
            ctx.accounts.admin.to_account_info(),
        ])?;
        Ok(())
    }
}

#[account]
pub struct State {
    pub admin: Pubkey,
    pub day: Pubkey,
    pub night: Pubkey,
    pub counter: u32,
}

#[derive(Accounts)]
pub struct Setup<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 32 + 32 + 4)]
    pub state: Account<'info, State>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Pay<'info> {
    #[account(mut)]
    pub state: Account<'info, State>,
    pub admin: Signer<'info>,
    #[account(mut)]
    pub treasury: Account<'info, TokenAccount>,
    #[account(mut)]
    pub recipient_ata: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub clock: Sysvar<'info, Clock>,
}
#[error_code] pub enum Errs { #[msg("admin mismatch")] Admin }
