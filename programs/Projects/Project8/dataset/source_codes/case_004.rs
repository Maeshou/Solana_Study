// Program 1: guild_treasury_pay
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

declare_id!("Gu1ldTreasuryPay111111111111111111111111");

#[program]
pub mod guild_treasury_pay {
    use super::*;

    pub fn init_vault(ctx: Context<InitVault>, seed_tag: u64) -> Result<()> {
        let v = &mut ctx.accounts.vault;
        v.admin = ctx.accounts.admin.key();
        v.tag = seed_tag.rotate_left(2).wrapping_add(17);
        v.rounds = 1;
        for _ in 0..3 {
            v.tag = v.tag.rotate_right(1).wrapping_mul(5).wrapping_add(11);
            v.rounds = v.rounds.saturating_add(((v.tag % 19) as u32) + 2);
        }
        Ok(())
    }

    pub fn pay(ctx: Context<Pay>, lamports: u64) -> Result<()> {
        let ix = system_instruction::transfer(&ctx.accounts.vault.key(), &ctx.accounts.recipient.key(), lamports);

        let bump = *ctx.bumps.get("vault").ok_or(error!(Errs::MissingBump))?;
        let seeds: &[&[u8]] = &[b"vault", ctx.accounts.admin.key.as_ref(), &ctx.accounts.vault.tag.to_le_bytes(), &[bump]];

        invoke_signed(
            &ix,
            &[
                ctx.accounts.vault.to_account_info(),
                ctx.accounts.recipient.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[seeds],
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitVault<'info> {
    #[account(
        init,
        payer = admin,
        space = 8 + 32 + 8 + 4,
        seeds=[b"vault", admin.key().as_ref(), seed_tag.to_le_bytes().as_ref()],
        bump
    )]
    pub vault: Account<'info, VaultState>,
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub seed_tag: u64,
}

#[derive(Accounts)]
pub struct Pay<'info> {
    #[account(
        mut,
        seeds=[b"vault", admin.key().as_ref(), vault.tag.to_le_bytes().as_ref()],
        bump
    )]
    pub vault: Account<'info, VaultState>,
    #[account(mut)]
    pub recipient: SystemAccount<'info>,
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct VaultState {
    pub admin: Pubkey,
    pub tag: u64,
    pub rounds: u32,
}

#[error_code]
pub enum Errs { #[msg("missing bump")] MissingBump }
