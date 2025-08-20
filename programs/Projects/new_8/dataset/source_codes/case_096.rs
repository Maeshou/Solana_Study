use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

declare_id!("BumpSeedCano111111111111111111111111111111");

#[program]
pub mod bump_seed_canonicalization_demo {
    use super::*;

    pub fn initialize_vault(ctx: Context<InitializeVault>) -> Result<()> {
        let v = &mut ctx.accounts.vault;
        v.owner = ctx.accounts.user.key();
        // Anchor が正しい bump を検証済み
        v.bump = *ctx.bumps.get("vault").unwrap();
        Ok(())
    }

    pub fn withdraw_with_custom_bump(
        ctx: Context<WithdrawWithCustomBump>,
        user_bump: u8,
        lamports: u64,
    ) -> Result<()> {
        let vault = &ctx.accounts.vault;

        // ここで「外部入力の bump(user_bump)」を seeds にそのまま使用している
        // → 検証済みの v.bump と一致する保証がない
        let seeds = &[
            b"vault".as_ref(),
            vault.owner.as_ref(),
            core::slice::from_ref(&user_bump),
        ];

        let derived = Pubkey::create_program_address(
            &[b"vault", vault.owner.as_ref(), &[user_bump]],
            ctx.program_id,
        ).map_err(|_| error!(VaultError::InvalidSeeds))?;

        let ix = system_instruction::transfer(
            &derived,
            &ctx.accounts.recipient.key(),
            lamports,
        );

        invoke_signed(
            &ix,
            &[
                ctx.accounts.vault_account.to_account_info(),
                ctx.accounts.recipient.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[seeds],
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 32 + 1,
        seeds = [b"vault", user.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, VaultState>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WithdrawWithCustomBump<'info> {
    #[account(
        mut,
        seeds = [b"vault", user.key().as_ref()],
        bump = vault.bump   // ← ここで検証はしている
    )]
    pub vault: Account<'info, VaultState>,
    /// CHECK: 検証されないアカウント
    pub vault_account: AccountInfo<'info>,
    #[account(mut)]
    pub recipient: AccountInfo<'info>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct VaultState {
    pub owner: Pubkey,
    pub bump: u8,
}

#[error_code]
pub enum VaultError {
    #[msg("Invalid seeds provided")]
    InvalidSeeds,
}
