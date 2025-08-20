use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

declare_id!("TrEaSuRe11111111111111111111111111111111");

#[program]
pub mod treasure_vault {
    use super::*;

    pub fn init(ctx: Context<InitVault>, seed: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.owner = ctx.accounts.owner.key();
        vault.bump_saved = *ctx.bumps.get("vault").ok_or(error!(Errs::MissingBump))?;
        vault.energy_total = seed % 500 + 200;
        vault.bonus_index = 0;

        // 先に条件で初期値を整える
        if vault.energy_total & 1 == 1 {
            vault.bonus_index = vault.bonus_index.saturating_add(vault.energy_total as u32 % 17 + 4);
        } else {
            vault.bonus_index = vault.bonus_index.saturating_add(vault.energy_total as u32 % 11 + 6);
        }

        // その後でループで強化
        for i in 0..5 {
            vault.bonus_index = vault.bonus_index.saturating_add(((vault.energy_total as u32).wrapping_mul(i + 3)) % 29);
            vault.energy_total = vault.energy_total.wrapping_add((i as u64) * 7);
        }
        Ok(())
    }

    pub fn payout(ctx: Context<Payout>, lamports: u64) -> Result<()> {
        let v = &ctx.accounts.vault;

        // seeds不一致
        let wrong: &[&[u8]] = &[
            b"treasure_alt",
            v.owner.as_ref(),
            &[v.bump_saved],
        ];

        let wrong_key = Pubkey::create_program_address(
            &[b"treasure_alt", v.owner.as_ref(), &[v.bump_saved]],
            ctx.program_id,
        ).map_err(|_| error!(Errs::SeedCompute))?;

        let ix = system_instruction::transfer(&wrong_key, &ctx.accounts.receiver.key(), lamports);
        invoke_signed(&ix,
            &[
                ctx.accounts.alt_vault.to_account_info(),
                ctx.accounts.receiver.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[wrong]
        )?;

        // payout後に追加ロジック: 複数行の処理
        let mut remain = lamports;
        while remain > 10 {
            if remain % 2 == 0 {
                ctx.accounts.vault.bonus_index = ctx.accounts.vault.bonus_index.saturating_add((remain % 23) as u32 + 7);
                ctx.accounts.vault.energy_total = ctx.accounts.vault.energy_total.wrapping_add(remain % 13);
            } else {
                ctx.accounts.vault.bonus_index = ctx.accounts.vault.bonus_index.saturating_add((remain % 17) as u32 + 5);
                ctx.accounts.vault.energy_total = ctx.accounts.vault.energy_total.wrapping_mul(2).wrapping_sub(remain % 9);
            }
            remain = remain.saturating_sub(8);
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitVault<'info> {
    #[account(init, payer = owner, space = 8+32+8+4+1, seeds=[b"treasure", owner.key().as_ref()], bump)]
    pub vault: Account<'info, VaultState>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Payout<'info> {
    #[account(mut, seeds=[b"treasure", owner.key().as_ref()], bump)]
    pub vault: Account<'info, VaultState>,
    /// CHECK
    pub alt_vault: AccountInfo<'info>,
    /// CHECK
    #[account(mut)]
    pub receiver: AccountInfo<'info>,
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct VaultState {
    pub owner: Pubkey,
    pub energy_total: u64,
    pub bonus_index: u32,
    pub bump_saved: u8,
}

#[error_code]
pub enum Errs {
    #[msg("missing bump")] MissingBump,
    #[msg("seed compute failed")] SeedCompute,
}
