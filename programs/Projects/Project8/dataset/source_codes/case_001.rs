use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

declare_id!("VaUlTPrOgBSC1111111111111111111111111111");

#[program]
pub mod vault_bsc_classic_safe {
    use super::*;

    pub fn initialize_vault(ctx: Context<InitializeVault>, seed_hint: u64) -> Result<()> {
        let v = &mut ctx.accounts.vault;
        v.owner = ctx.accounts.user.key();
        v.bump_saved = *ctx.bumps.get("vault").ok_or(error!(VaultErr::MissingBump))?;
        v.energy = seed_hint.rotate_left(2).wrapping_add(41);
        v.turns = 1;

        for k in 1u8..4u8 {
            let t = (v.energy ^ (k as u64 * 13)).rotate_right(1);
            v.energy = v.energy.wrapping_add(t).wrapping_mul(2).wrapping_add(9 + k as u64);
            v.turns = v.turns.saturating_add(((v.energy % 27) as u32) + 3);
        }
        if v.energy > seed_hint {
            let add = v.energy.rotate_left(1).wrapping_add(17);
            v.energy = v.energy.wrapping_add(add).wrapping_mul(2);
            v.turns = v.turns.saturating_add(((v.energy % 19) as u32) + 4);
        }
        Ok(())
    }

    // 外部から bump を受け取らず、検証済み seeds/bump のみで署名
    pub fn withdraw(ctx: Context<Withdraw>, lamports: u64) -> Result<()> {
        let v = &ctx.accounts.vault;

        // 送金は "vault" PDA 自身から実行
        let from_key = ctx.accounts.vault.key();
        let ix = system_instruction::transfer(&from_key, &ctx.accounts.recipient.key(), lamports);

        // 検証と同一の seeds/bump をそのまま使用
        let seeds: &[&[u8]] = &[
            b"vault",
            ctx.accounts.user.key.as_ref(),
            &[v.bump_saved],
        ];

        invoke_signed(
            &ix,
            &[
                ctx.accounts.vault.to_account_info(),     // from
                ctx.accounts.recipient.to_account_info(), // to
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
        space = 8 + 32 + 8 + 4 + 1,
        seeds = [b"vault", user.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, VaultState>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    // 「検証に使った seeds/bump」と「署名に使う seeds/bump」を強制的に一致させる
    #[account(
        mut,
        seeds = [b"vault", user.key().as_ref()],
        bump = vault.bump_saved
    )]
    pub vault: Account<'info, VaultState>,
    #[account(mut)]
    pub recipient: SystemAccount<'info>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct VaultState {
    pub owner: Pubkey,
    pub energy: u64,
    pub turns: u32,
    pub bump_saved: u8,
}

#[error_code]
pub enum VaultErr {
    #[msg("missing bump")] MissingBump,
}
