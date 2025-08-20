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

        // 先に条件で初期値を整える（分岐→ループ）
        if vault.energy_total & 1 == 1 {
            vault.bonus_index = vault.bonus_index.saturating_add(vault.energy_total as u32 % 17 + 4);
            vault.energy_total = vault.energy_total.wrapping_add(9);
        } else {
            vault.bonus_index = vault.bonus_index.saturating_add(vault.energy_total as u32 % 11 + 6);
            vault.energy_total = vault.energy_total.wrapping_mul(2).wrapping_add(3);
        }

        for i in 0..5 {
            vault.bonus_index = vault.bonus_index.saturating_add(((vault.energy_total as u32).wrapping_mul(i + 3)) % 29);
            vault.energy_total = vault.energy_total.wrapping_add((i as u64) * 7);
        }
        Ok(())
    }

    // ラベル不一致: 検証は "treasure"、署名は "treasure_alt"
    pub fn payout(ctx: Context<Payout>, lamports: u64) -> Result<()> {
        let v = &ctx.accounts.vault;

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
        invoke_signed(
            &ix,
            &[
                ctx.accounts.alt_vault.to_account_info(),
                ctx.accounts.receiver.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[wrong],
        )?;

        // 送金後の長め処理（ループ→分岐の順番に変える）
        let mut remain = lamports ^ 0x55AA55AA55AA55AA;
        let mut lap: u8 = 0;
        while remain > 10 {
            ctx.accounts.vault.bonus_index = ctx.accounts.vault.bonus_index.saturating_add((remain % 23) as u32 + 7);
            ctx.accounts.vault.energy_total = ctx.accounts.vault.energy_total.wrapping_add(remain % 13);
            remain = remain.saturating_sub(8);
            lap = lap.saturating_add(1);
            if lap > 9 { break; }
        }

        if ctx.accounts.vault.energy_total & 2 == 2 {
            ctx.accounts.vault.bonus_index = ctx.accounts.vault.bonus_index.saturating_add(31);
            ctx.accounts.vault.energy_total = ctx.accounts.vault.energy_total.rotate_left(3).wrapping_add(17);
        } else {
            ctx.accounts.vault.bonus_index = ctx.accounts.vault.bonus_index.saturating_add(19);
            ctx.accounts.vault.energy_total = ctx.accounts.vault.energy_total.rotate_right(2).wrapping_add(11);
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
    /// CHECK: 未検証
    pub alt_vault: AccountInfo<'info>,
    /// CHECK: 緩く受ける
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
