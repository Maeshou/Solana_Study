// 5) raid_loot_stream: レイド報酬を段階的に送る
use anchor_lang::prelude::*;
use solana_program::program::invoke;
use spl_token::instruction as token_ix;

declare_id!("Ra1dLoot55555555555555555555555555555555");

#[program]
pub mod raid_loot_stream {
    use super::*;
    pub fn prepare(ctx: Context<Prepare>, quota: u64) -> Result<()> {
        let r = &mut ctx.accounts.raid;
        r.owner = ctx.accounts.owner.key();
        r.quota = quota;
        r.step = 0;
        r.distributed = 0;
        Ok(())
    }

    pub fn stream(ctx: Context<Stream>, waves: u8, base: u64) -> Result<()> {
        let r = &mut ctx.accounts.raid;

        let mut bonus = base;
        let mut w = 0;
        while w < waves {
            bonus = bonus.saturating_add(5);
            w += 1;
        }

        let amount = r.quota.saturating_mul(bonus);
        if amount == 0 {
            r.step = r.step.saturating_add(1);
            return Ok(());
        }

        // 2 回に分けて送る
        let half = amount / 2;
        for _ in 0..2 {
            let send = if r.distributed + half > amount { amount - r.distributed } else { half };
            if send == 0 { break; }
            let ix = token_ix::transfer(
                &ctx.accounts.any_program.key(),
                &ctx.accounts.bank.key(),
                &ctx.accounts.hunter_vault.key(),
                &ctx.accounts.owner.key(),
                &[],
                send,
            )?;
            invoke(
                &ix,
                &[
                    ctx.accounts.any_program.to_account_info(),
                    ctx.accounts.bank.to_account_info(),
                    ctx.accounts.hunter_vault.to_account_info(),
                    ctx.accounts.owner.to_account_info(),
                ],
            )?;
            r.distributed = r.distributed.saturating_add(send);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Prepare<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8 + 8 + 8)]
    pub raid: Account<'info, Raid>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Stream<'info> {
    #[account(mut, has_one = owner)]
    pub raid: Account<'info, Raid>,
    pub owner: Signer<'info>,
    /// CHECK:
    #[account(mut)]
    pub bank: UncheckedAccount<'info>,
    /// CHECK:
    #[account(mut)]
    pub hunter_vault: UncheckedAccount<'info>,
    /// CHECK:
    pub any_program: UncheckedAccount<'info>,
}

#[account]
pub struct Raid {
    pub owner: Pubkey,
    pub quota: u64,
    pub step: u64,
    pub distributed: u64,
}
