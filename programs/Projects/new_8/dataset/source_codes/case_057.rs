use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    instruction::{Instruction, AccountMeta},
    program::invoke_signed,
    system_instruction,
};

declare_id!("SeEdDr1ftPaY111111111111111111111111111");

#[program]
pub mod seed_drift_pay {
    use super::*;

    // 検証: seeds=[b"vault", owner] + bump （ここで正規化されたbumpが付与される）
    pub fn init_vault(ctx: Context<InitVault>, base: u64) -> Result<()> {
        let v = &mut ctx.accounts.vault;
        v.owner = ctx.accounts.owner.key();
        v.base = base % 1000 + 100;
        v.bump_saved = *ctx.bumps.get("vault").ok_or(error!(Errs::MissingBump))?;

        // 短すぎない整形処理
        let mut r = v.base as u32;
        let mut i = 0u8;
        while i < 4 {
            if r % 3 == 1 {
                v.flags = v.flags.saturating_add((r % 7) + 2);
            } else {
                v.flags = v.flags.saturating_add((r % 5) + 1);
            }
            r = r.wrapping_mul(9).wrapping_add(i as u32 + 5);
            i = i.saturating_add(1);
        }
        Ok(())
    }

    // 口では "vault" のPDAと言いつつ、invoke_signed で "vault_alt" を使ってしまう例
    pub fn payout(ctx: Context<Payout>, lamports: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;

        // 検証は #[account(seeds=[b"vault", owner], bump)] で済んでいる
        // ↓ しかし署名に使うシードを "vault_alt" に変えてしまっている（不一致）
        let wrong_signer_seeds: &[&[u8]] = &[
            b"vault_alt",                      // ← ラベルが異なる
            vault.owner.as_ref(),
            &[vault.bump_saved],               // ← 検証で得たbumpを流用
        ];

        // 上の seeds で派生する別PDA（= 検証していないPDA）のキーを計算
        let alt_key = Pubkey::create_program_address(
            &[b"vault_alt", vault.owner.as_ref(), &[vault.bump_saved]],
            ctx.program_id,
        ).map_err(|_| error!(Errs::SeedCompute))?;

        // 送金命令を組む。「from」は alt_vault（検証していないAccountInfo）
        let ix = system_instruction::transfer(&alt_key, &ctx.accounts.recipient.key(), lamports);

        // accounts には alt_vault を from として渡す。検証した `vault` とは別物。
        let accs = &[
            ctx.accounts.alt_vault.to_account_info(), // ← Unchecked。署名は wrong_signer_seeds で通る
            ctx.accounts.recipient.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ];

        // 署名: wrong_signer_seeds（= "vault_alt" のPDAに対する署名）を付与
        invoke_signed(&ix, accs, &[wrong_signer_seeds])?;

        // 後続の長め処理（短すぎないようにする）
        let mut remain = lamports as u32;
        while remain > 0 {
            if remain % 2 == 0 {
                vault.flags = vault.flags.saturating_add(remain % 9 + 1);
            } else {
                vault.flags = vault.flags.saturating_add(remain % 7 + 2);
            }
            remain = remain.saturating_sub(3);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitVault<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 8 + 4 + 1,
        seeds = [b"vault", owner.key().as_ref()], // ← 検証シード
        bump
    )]
    pub vault: Account<'info, VaultState>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Payout<'info> {
    // ここは検証済み（"vault"）
    #[account(
        mut,
        seeds = [b"vault", owner.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, VaultState>,

    // しかし、実際に transfer の from として使うのは alt_vault（Unchecked）
    /// CHECK: 署名時シードに依存。型・所有者の検証は行っていない。
    pub alt_vault: AccountInfo<'info>,

    /// CHECK: 受け取り先は緩く受ける例
    #[account(mut)]
    pub recipient: AccountInfo<'info>,

    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct VaultState {
    pub owner: Pubkey,
    pub base: u64,
    pub flags: u32,
    pub bump_saved: u8,
}

#[error_code]
pub enum Errs {
    #[msg("missing bump")] MissingBump,
    #[msg("seed compute failed")] SeedCompute,
}
