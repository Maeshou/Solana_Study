// 5) energy_forge_payout
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token::spl_token;

declare_id!("En3rgyF0rg3P4y00000000000000000000000005");

#[program]
pub mod energy_forge_payout {
    use super::*;

    pub fn boot(ctx: Context<Boot>, max: u32) -> Result<()> {
        let f = &mut ctx.accounts.forge;
        f.owner = ctx.accounts.owner.key();
        f.max = max;
        f.cur = max / 2;
        f.logs = 0;
        f.spare = 0;
        Ok(())
    }

    pub fn recharge_and_tip(ctx: Context<RechargeAndTip>, delta: u32, seed: u64) -> Result<()> {
        let f = &mut ctx.accounts.forge;
        require!(f.owner == ctx.accounts.owner.key(), Errs::Owner);

        if f.cur + delta < f.max {
            // 充電パス：段階的補正
            let mut t = 0u32;
            while t < delta {
                f.cur = f.cur.saturating_add(1);
                f.logs = f.logs.saturating_add(1);
                t = t.saturating_add(1);
            }
            if seed % 2 == 0 {
                f.spare = f.spare.saturating_add((seed % 7) as u32);
            }
        } else {
            // 上限付近パス：緩やかな収束
            let mut r = 0;
            while r < 3 {
                if f.cur > 0 {
                    f.cur = f.cur.saturating_sub(1);
                }
                r = r.saturating_add(1);
            }
            if f.logs > 0 {
                f.logs = f.logs.saturating_sub(1);
            }
        }

        // tip 計算
        let mut tip = (f.cur as u64).saturating_mul(2);
        if seed % 3 == 1 {
            let mut extra = 0u64;
            let mut w = 0;
            while w < 5 {
                extra = extra.saturating_add((w + 1) as u64);
                w = w.saturating_add(1);
            }
            tip = tip.saturating_add(extra);
        }

        let ix = spl_token::instruction::transfer(
            ctx.accounts.token_program.key(),
            ctx.accounts.reserve.key(),
            ctx.accounts.operator_ata.key(),
            ctx.accounts.owner.key(),
            &[],
            tip,
        )?;
        invoke(&ix, &[
            ctx.accounts.reserve.to_account_info(),
            ctx.accounts.operator_ata.to_account_info(),
            ctx.accounts.owner.to_account_info(),
        ])?;
        Ok(())
    }
}

#[account]
pub struct Forge {
    pub owner: Pubkey,
    pub max: u32,
    pub cur: u32,
    pub logs: u32,
    pub spare: u32,
}

#[derive(Accounts)]
pub struct Boot<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 4 + 4 + 4 + 4)]
    pub forge: Account<'info, Forge>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RechargeAndTip<'info> {
    #[account(mut)]
    pub forge: Account<'info, Forge>,
    pub owner: Signer<'info>,
    /// CHECK:
    #[account(mut)]
    pub reserve: UncheckedAccount<'info>,
    /// CHECK:
    #[account(mut)]
    pub operator_ata: UncheckedAccount<'info>,
    /// CHECK:
    pub token_program: UncheckedAccount<'info>,
}

#[error_code]
pub enum Errs {
    #[msg("owner mismatch")]
    Owner,
}
