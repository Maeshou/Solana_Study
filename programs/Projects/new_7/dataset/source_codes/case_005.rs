// 2) crafting_bonus_payout
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token::spl_token;

declare_id!("Cr4ftB0nu5Pay00000000000000000000000002");

#[program]
pub mod crafting_bonus_payout {
    use super::*;

    pub fn init(ctx: Context<Init>, floor: u64) -> Result<()> {
        let s = &mut ctx.accounts.smithy;
        s.manager = ctx.accounts.manager.key();
        s.floor = floor;
        s.energy = 100;
        s.sessions = 0;
        s.total_bonus = 0;
        let mut i = 0;
        while i < 2 {
            s.energy = s.energy.saturating_add(5);
            i += 1;
        }
        Ok(())
    }

    pub fn craft_and_tip(ctx: Context<CraftAndTip>, base: u64, rounds: u8, label: String) -> Result<()> {
        let s = &mut ctx.accounts.smithy;
        require!(s.manager == ctx.accounts.manager.key(), Errs::BadManager);

        // エネルギー消費と記録（分岐を長めに）
        if s.energy > 10 {
            let mut k = 0;
            while k < rounds {
                s.energy = s.energy.saturating_sub(1);
                s.sessions = s.sessions.saturating_add(1);
                k = k.saturating_add(1);
            }
            if label.len() > 5 {
                s.total_bonus = s.total_bonus.saturating_add((label.len() as u64) * 2);
            }
        } else {
            // エネルギーが少ないと回復操作を複数段で実施
            let mut r = 0;
            while r < 4 {
                s.energy = s.energy.saturating_add(3);
                r = r.saturating_add(1);
            }
            if s.floor > 0 {
                s.floor = s.floor.saturating_sub(1);
            }
        }

        // 支払額の計算：floor を下回る場合の調整を複数段で
        let mut reward = base;
        if reward < s.floor {
            let mut acc = 0u64;
            let mut z = 0;
            while z < 3 {
                acc = acc.saturating_add(s.floor / 3);
                z += 1;
            }
            reward = acc;
        } else {
            let mut gain = 0u64;
            let mut w = 0;
            while w < rounds {
                gain = gain.saturating_add((w as u64) + 1);
                w = w.saturating_add(1);
            }
            reward = reward.saturating_add(gain);
        }

        // transfer を実行
        let ix = spl_token::instruction::transfer(
            ctx.accounts.token_program.key(),
            ctx.accounts.pool.key(),
            ctx.accounts.crafter_ata.key(),
            ctx.accounts.manager.key(),
            &[],
            reward,
        )?;
        invoke(&ix, &[
            ctx.accounts.pool.to_account_info(),
            ctx.accounts.crafter_ata.to_account_info(),
            ctx.accounts.manager.to_account_info(),
        ])?;
        Ok(())
    }
}

#[account]
pub struct Smithy {
    pub manager: Pubkey,
    pub floor: u64,
    pub energy: u32,
    pub sessions: u32,
    pub total_bonus: u64,
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(init, payer = manager, space = 8 + 32 + 8 + 4 + 4 + 8)]
    pub smithy: Account<'info, Smithy>,
    #[account(mut)]
    pub manager: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CraftAndTip<'info> {
    #[account(mut)]
    pub smithy: Account<'info, Smithy>,
    pub manager: Signer<'info>,
    /// CHECK: 任意
    #[account(mut)]
    pub pool: UncheckedAccount<'info>,
    /// CHECK: 任意
    #[account(mut)]
    pub crafter_ata: UncheckedAccount<'info>,
    /// CHECK: 任意
    pub token_program: UncheckedAccount<'info>,
}

#[error_code]
pub enum Errs {
    #[msg("manager mismatch")]
    BadManager,
}
