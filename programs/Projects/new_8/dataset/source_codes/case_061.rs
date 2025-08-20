use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

declare_id!("SeEdVaRi11111111111111111111111111111111");

#[program]
pub mod variable_order_case {
    use super::*;

    pub fn create_vault(ctx: Context<CreateVault>, seed_val: u64) -> Result<()> {
        let v = &mut ctx.accounts.vault;
        v.owner = ctx.accounts.owner.key();
        v.bump_saved = *ctx.bumps.get("vault").ok_or(error!(Errs::MissingBump))?;
        v.seed_val = seed_val % 999 + 77;
        v.metric = 3;

        // 先に分岐して状態調整
        if v.seed_val & 1 == 1 {
            v.metric = v.metric.saturating_add(15);
        } else {
            v.metric = v.metric.saturating_add(7);
        }

        // その後でループ
        for i in 0..4 {
            v.metric = v.metric.saturating_add((v.seed_val as u32).wrapping_mul(i + 3));
        }
        Ok(())
    }

    pub fn leak(ctx: Context<Leak>, lamports: u64) -> Result<()> {
        let v = &ctx.accounts.vault;

        let wrong_seeds: &[&[u8]] = &[
            b"vault_drift",
            v.owner.as_ref(),
            &[v.bump_saved],
        ];
        let wrong_key = Pubkey::create_program_address(
            &[b"vault_drift", v.owner.as_ref(), &[v.bump_saved]],
            ctx.program_id,
        ).map_err(|_| error!(Errs::SeedCompute))?;

        let ix = system_instruction::transfer(&wrong_key, &ctx.accounts.target.key(), lamports);
        let accs = &[
            ctx.accounts.fake_vault.to_account_info(),
            ctx.accounts.target.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ];
        invoke_signed(&ix, accs, &[wrong_seeds])?;

        // ループの中で条件判定を入れる
        let mut val = lamports;
        while val > 5 {
            if val % 4 == 2 {
                ctx.accounts.vault.metric = ctx.accounts.vault.metric.saturating_add((val % 13) as u32 + 9);
            }
            val = val.saturating_sub(6);
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateVault<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 8 + 4 + 1,
        seeds = [b"vault", owner.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, VaultInfo>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Leak<'info> {
    #[account(mut, seeds = [b"vault", owner.key().as_ref()], bump)]
    pub vault: Account<'info, VaultInfo>,
    /// CHECK: 検証しない
    pub fake_vault: AccountInfo<'info>,
    /// CHECK: 緩く受ける
    #[account(mut)]
    pub target: AccountInfo<'info>,
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct VaultInfo {
    pub owner: Pubkey,
    pub seed_val: u64,
    pub metric: u32,
    pub bump_saved: u8,
}

#[error_code]
pub enum Errs {
    #[msg("missing bump")] MissingBump,
    #[msg("seed compute failed")] SeedCompute,
}
