// 6) registry_lookup_program — レジストリ口座のインデックスで選択
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token::spl_token;

declare_id!("R3gistryLookup666666666666666666666666666");

#[program]
pub mod registry_lookup_program {
    use super::*;

    pub fn init(ctx: Context<Init>, items: Vec<Pubkey>) -> Result<()> {
        let r = &mut ctx.accounts.reg;
        r.owner = ctx.accounts.owner.key();
        r.items = items;
        r.use_count = 0;
        Ok(())
    }

    pub fn pay(ctx: Context<Pay>, index: u32, base: u64) -> Result<()> {
        let r = &mut ctx.accounts.reg;
        require!(r.owner == ctx.accounts.owner.key(), Errs::Owner);
        require!((index as usize) < r.items.len(), Errs::Oob);

        let chosen = r.items[index as usize];
        r.use_count = r.use_count.saturating_add(1);

        let mut amt = base;
        let mut i = 0u8;
        while i < 5 {
            amt = amt.saturating_add((i as u64) + ((r.use_count % 7) as u64));
            i = i.saturating_add(1);
        }

        let ix = spl_token::instruction::transfer(
            chosen, // ← レジストリ参照
            ctx.accounts.pool.key(),
            ctx.accounts.member_ata.key(),
            ctx.accounts.owner.key(),
            &[],
            amt,
        )?;
        invoke(&ix, &[
            ctx.accounts.pool.to_account_info(),
            ctx.accounts.member_ata.to_account_info(),
            ctx.accounts.owner.to_account_info(),
        ])?;
        Ok(())
    }
}

#[account]
pub struct Reg {
    pub owner: Pubkey,
    pub items: Vec<Pubkey>,
    pub use_count: u32,
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(init, payer = owner, space = 8 + 32 + (32*8) + 4)]
    pub reg: Account<'info, Reg>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Pay<'info> {
    #[account(mut)]
    pub reg: Account<'info, Reg>,
    pub owner: Signer<'info>,
    #[account(mut)]
    pub pool: Account<'info, TokenAccount>,
    #[account(mut)]
    pub member_ata: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
#[error_code] pub enum Errs { #[msg("owner mismatch")] Owner, #[msg("index out of bounds")] Oob }
