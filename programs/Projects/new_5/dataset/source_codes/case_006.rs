use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWp1jZQzZ1u5wzT1ZtziB6rL9eWp");

#[program]
pub mod duplicate_mutable {
    use super::*;

    /// 同一型の vault_a, vault_b を mutable として受け取るが、
    /// 両者が同じキーかどうかをチェックしていない！
    pub fn transfer_between_vaults(
        ctx: Context<TransferBetweenVaults>,
        amount: u64,
    ) -> ProgramResult {
        let vault_a = &mut ctx.accounts.vault_a;
        let vault_b = &mut ctx.accounts.vault_b;

        // ❌ 本来はここで vault_a.key() != vault_b.key() をチェックすべき
        // require!(
        //     vault_a.key() != vault_b.key(),
        //     DuplicateMutableAccount
        // );

        // 送金ロジック
        **vault_a.to_account_info().try_borrow_mut_lamports()? -= amount;
        **vault_b.to_account_info().try_borrow_mut_lamports()? += amount;
        Ok(())
    }
}

/// Duplicate Mutable Account 脆弱性を表すカスタムエラー
#[error]
pub enum ErrorCode {
    #[msg("Both mutable accounts must be different.")]
    DuplicateMutableAccount,
}

#[derive(Accounts)]
pub struct TransferBetweenVaults<'info> {
    /// 同じ型のアカウントを mut として二つ受け取る
    #[account(mut)]
    pub vault_a: Account<'info, Vault>,

    #[account(mut)]
    pub vault_b: Account<'info, Vault>,

    #[account(signer)]
    /// 送金権限を持つ署名アカウント
    pub authority: Signer<'info>,

    /// システムプログラム
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault {
    /// この vault のオーナー
    pub owner: Pubkey,
}
