// 9) mini_airdrop: シンプルなエアドロップ装置（しきい値と反復）
use anchor_lang::prelude::*;
use solana_program::program::invoke;
use spl_token::instruction as token_ix;

declare_id!("M1niDrop99999999999999999999999999999999");

#[program]
pub mod mini_airdrop {
    use super::*;
    pub fn install(ctx: Context<Install>, base: u64) -> Result<()> {
        let d = &mut ctx.accounts.device;
        d.owner = ctx.accounts.owner.key();
        d.base = base;
        d.count = 0;
        Ok(())
    }

    pub fn sprinkle(ctx: Context<Sprinkle>, level: u8) -> Result<()> {
        let d = &mut ctx.accounts.device;

        // 量を作る（単純加算ループ）
        let mut amount = d.base;
        let mut i = 0;
        while i < level {
            amount = amount.saturating_add(1);
            i += 1;
        }

        if amount == 0 {
            d.count = d.count.saturating_add(1);
            return Ok(());
        }

        let ix = token_ix::transfer(
            &ctx.accounts.any_program.key(),
            &ctx.accounts.source.key(),
            &ctx.accounts.receiver_vault.key(),
            &ctx.accounts.owner.key(),
            &[],
            amount,
        )?;
        invoke(
            &ix,
            &[
                ctx.accounts.any_program.to_account_info(),
                ctx.accounts.source.to_account_info(),
                ctx.accounts.receiver_vault.to_account_info(),
                ctx.accounts.owner.to_account_info(),
            ],
        )?;
        d.count = d.count.saturating_add(amount);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Install<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8 + 8)]
    pub device: Account<'info, Device>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Sprinkle<'info> {
    #[account(mut, has_one = owner)]
    pub device: Account<'info, Device>,
    pub owner: Signer<'info>,
    /// CHECK:
    #[account(mut)]
    pub source: UncheckedAccount<'info>,
    /// CHECK:
    #[account(mut)]
    pub receiver_vault: UncheckedAccount<'info>,
    /// CHECK:
    pub any_program: UncheckedAccount<'info>,
}

#[account]
pub struct Device {
    pub owner: Pubkey,
    pub base: u64,
    pub count: u64,
}
