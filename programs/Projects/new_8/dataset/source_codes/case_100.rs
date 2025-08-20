use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

// ─────────────────────────────────────────────────────────────
// Program 1: user_bump を直接使って署名（classic BSC）
// 検証: [b"vault", user]
// 署名: [b"vault", user, user_bump]（外部入力 bump をそのまま使用）
// ─────────────────────────────────────────────────────────────
declare_id!("VaUlTPrOgBSC1111111111111111111111111111");

#[program]
pub mod vault_bsc_classic {
    use super::*;

    pub fn initialize_vault(ctx: Context<InitializeVault>, seed_hint: u64) -> Result<()> {
        let v = &mut ctx.accounts.vault;
        v.owner = ctx.accounts.user.key();
        v.bump_saved = *ctx.bumps.get("vault").ok_or(error!(VaultErr::MissingBump))?;
        v.energy = seed_hint.rotate_left(2).wrapping_add(41);
        v.turns = 1;

        // 適当な計算（単純でよいが少し長め）
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

    pub fn withdraw_with_user_bump(
        ctx: Context<WithdrawWithUserBump>,
        user_bump: u8,
        lamports: u64,
    ) -> Result<()> {
        let v = &ctx.accounts.vault;

        // ここが BSC：検証済み bump と関係ない外部入力 bump を seeds にそのまま使って署名
        let seeds = &[
            b"vault".as_ref(),
            v.owner.as_ref(),
            core::slice::from_ref(&user_bump),
        ];
        let derived = Pubkey::create_program_address(
            &[b"vault", v.owner.as_ref(), &[user_bump]],
            ctx.program_id,
        ).map_err(|_| error!(VaultErr::SeedCompute))?;

        let ix = system_instruction::transfer(&derived, &ctx.accounts.recipient.key(), lamports);
        invoke_signed(
            &ix,
            &[
                ctx.accounts.vault_pda_hint.to_account_info(), // CHECK
                ctx.accounts.recipient.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[seeds],
        )?;

        // 処理を少しだけ続ける（短すぎないように）
        if lamports > 500 {
            let mut i = 1u8;
            let mut acc = lamports.rotate_left(2);
            while i < 3 {
                let z = (acc ^ (i as u64 * 21)).rotate_right(1);
                acc = acc.wrapping_add(z);
                i = i.saturating_add(1);
            }
        }

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
pub struct WithdrawWithUserBump<'info> {
    #[account(
        mut,
        seeds = [b"vault", user.key().as_ref()],
        bump = vault.bump_saved // 検証はしているが…
    )]
    pub vault: Account<'info, VaultState>,
    /// CHECK: 未検証のヒント口座
    pub vault_pda_hint: AccountInfo<'info>,
    #[account(mut)]
    pub recipient: AccountInfo<'info>,
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
    #[msg("seed compute failed")] SeedCompute,
}
