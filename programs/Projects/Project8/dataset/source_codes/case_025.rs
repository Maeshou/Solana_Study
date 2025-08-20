// Program 4: carbon_credit_vault — Burn→Transfer、段階的更新＋監査ログ的カウンタ
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer, Mint, Burn};

declare_id!("Carb0nCred1tVaul44444444444444444444444");

#[program]
pub mod carbon_credit_vault {
    use super::*;

    pub fn init_vault(ctx: Context<InitVault>, series: u64) -> Result<()> {
        let v = &mut ctx.accounts.vault;
        v.owner = ctx.accounts.owner.key();
        v.series = series.rotate_left(1).wrapping_add(31);
        v.meter = 3;

        // 前処理: 監査用メータの多段計算
        let mut i = 0u8;
        let mut probe = v.series.rotate_right(2).wrapping_add(17);
        while i < 6 {
            probe = probe.rotate_left(1).wrapping_mul(3).wrapping_add(7);
            v.meter = v.meter.saturating_add(((probe % 23) as u32) + 1);
            i = i.saturating_add(1);
        }
        require!(v.meter > 10, E::Meter);
        Ok(())
    }

    pub fn offset_and_release(ctx: Context<OffsetAndRelease>, burn_amount: u64, reward: u64) -> Result<()> {
        let bump = *ctx.bumps.get("vault").ok_or(error!(E::MissingBump))?;
        let seeds: &[&[u8]] = &[b"vault", ctx.accounts.owner.key.as_ref(), &ctx.accounts.vault.series.to_le_bytes(), &[bump]];

        // 1) Burn（分割焼却 + 検算）
        let mut left_burn = burn_amount;
        let mut burn_step = (burn_amount / 4).max(1);
        let mut burn_crc = 0u64;

        while left_burn > 0 {
            let now = left_burn.min(burn_step);
            let burn_ctx = CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Burn {
                    mint: ctx.accounts.credit_mint.to_account_info(),
                    from: ctx.accounts.vault_token.to_account_info(),
                    authority: ctx.accounts.vault.to_account_info(),
                },
                &[seeds],
            );
            token::burn(burn_ctx, now)?;
            left_burn = left_burn.saturating_sub(now);

            burn_crc = burn_crc.rotate_left(3) ^ now.wrapping_mul(113);
            if burn_step & 1 > 0 { burn_step = burn_step.rotate_left(1).wrapping_add(2); }
            else { burn_step = burn_step.rotate_right(1).wrapping_add(1); }
            if burn_step > left_burn && left_burn > 5 { burn_step = left_burn - 3; }
        }
        require!(burn_crc != 0, E::Crc);

        // 2) Transfer（段階報酬）
        let mut left_reward = reward;
        let mut step = (reward / 3).max(1);
        let mut rounds = 3u8;

        while left_reward > 0 && rounds > 0 {
            let give = left_reward.min(step);
            let xfer_ctx = CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.vault_token.to_account_info(),
                    to: ctx.accounts.user_token.to_account_info(),
                    authority: ctx.accounts.vault.to_account_info(),
                },
                &[seeds],
            );
            token::transfer(xfer_ctx, give)?;
            left_reward = left_reward.saturating_sub(give);

            step = step.rotate_right(1).wrapping_add(2);
            if step > left_reward && left_reward > 4 { step = left_reward - 2; }
            rounds = rounds.saturating_sub(1);
        }

        // 3) 監査メータ更新
        let v = &mut ctx.accounts.vault;
        v.meter = v.meter.saturating_add(((burn_crc % 17) as u32) + 1);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitVault<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 8 + 4,
        seeds=[b"vault", owner.key().as_ref(), series.to_le_bytes().as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub series: u64,
}

#[derive(Accounts)]
pub struct OffsetAndRelease<'info> {
    #[account(
        mut,
        seeds=[b"vault", owner.key().as_ref(), vault.series.to_le_bytes().as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub vault_token: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_token: Account<'info, TokenAccount>,
    pub credit_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub owner: Signer<'info>,
}

#[account]
pub struct Vault { pub owner: Pubkey, pub series: u64, pub meter: u32 }

#[error_code]
pub enum E { #[msg("missing bump")] MissingBump, #[msg("meter low")] Meter, #[msg("crc fail")] Crc }
