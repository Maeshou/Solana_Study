// 10) treasury_rebate: 手数料の一部をリベート（ステップ処理と終端判定）
use anchor_lang::prelude::*;
use solana_program::program::invoke;
use spl_token::instruction as token_ix;

declare_id!("Tre4suryRebateAAAAAAAAAAAAAAAAAAAAAAAAAAA");

#[program]
pub mod treasury_rebate {
    use super::*;
    pub fn setup(ctx: Context<Setup>, fee_bps: u16) -> Result<()> {
        let t = &mut ctx.accounts.treasury;
        t.controller = ctx.accounts.controller.key();
        t.fee_bps = fee_bps.min(2000);
        t.rebated = 0;
        t.steps = 0;
        Ok(())
    }

    pub fn rebate(ctx: Context<Rebate>, base: u64, rounds: u8) -> Result<()> {
        let t = &mut ctx.accounts.treasury;

        let fee = base.saturating_mul(t.fee_bps as u64) / 10_000;
        let mut give = if base > fee { base - fee } else { 0 };

        if give == 0 {
            t.steps = t.steps.saturating_add(1);
            return Ok(());
        }

        let mut r = 0;
        while r < rounds {
            // 少しずつ減らして送付
            let part = give / 3;
            if part == 0 { break; }

            let ix = token_ix::transfer(
                &ctx.accounts.any_program.key(),
                &ctx.accounts.vault.key(),
                &ctx.accounts.client_vault.key(),
                &ctx.accounts.controller.key(),
                &[],
                part,
            )?;
            invoke(
                &ix,
                &[
                    ctx.accounts.any_program.to_account_info(),
                    ctx.accounts.vault.to_account_info(),
                    ctx.accounts.client_vault.to_account_info(),
                    ctx.accounts.controller.to_account_info(),
                ],
            )?;

            give = give.saturating_sub(part);
            t.rebated = t.rebated.saturating_add(part);
            r = r.saturating_add(1);
        }

        if give > 0 {
            // 残りを一括
            let ix2 = token_ix::transfer(
                &ctx.accounts.any_program.key(),
                &ctx.accounts.vault.key(),
                &ctx.accounts.client_vault.key(),
                &ctx.accounts.controller.key(),
                &[],
                give,
            )?;
            invoke(
                &ix2,
                &[
                    ctx.accounts.any_program.to_account_info(),
                    ctx.accounts.vault.to_account_info(),
                    ctx.accounts.client_vault.to_account_info(),
                    ctx.accounts.controller.to_account_info(),
                ],
            )?;
            t.rebated = t.rebated.saturating_add(give);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Setup<'info> {
    #[account(init, payer = controller, space = 8 + 32 + 2 + 8 + 8)]
    pub treasury: Account<'info, Treasury>,
    #[account(mut)]
    pub controller: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Rebate<'info> {
    #[account(mut, has_one = controller)]
    pub treasury: Account<'info, Treasury>,
    pub controller: Signer<'info>,
    /// CHECK:
    #[account(mut)]
    pub vault: UncheckedAccount<'info>,
    /// CHECK:
    #[account(mut)]
    pub client_vault: UncheckedAccount<'info>,
    /// CHECK:
    pub any_program: UncheckedAccount<'info>,
}

#[account]
pub struct Treasury {
    pub controller: Pubkey,
    pub fee_bps: u16,
    pub rebated: u64,
    pub steps: u64,
}
