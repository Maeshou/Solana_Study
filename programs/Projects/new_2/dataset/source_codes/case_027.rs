use anchor_lang::prelude::*;

declare_id!("ProgMixedOwner014IDXNOCPIVAR001");

#[program]
pub mod case_014_nocpi {
    use super::*;

    /// パターンA：has_one + signer で完全ガード
    pub fn enforced_deposit(ctx: Context<EnforcedDeposit>, amount: u64) -> Result<()> {
        let vault_info = ctx.accounts.vault_acc.to_account_info();
        let before = **vault_info.try_borrow_lamports()?;
        **vault_info.try_borrow_mut_lamports()? = before + amount;
        msg!("enforced_deposit 完了");
        Ok(())
    }

    /// パターンB：署名のみ検証（owner チェックなし）
    pub fn auth_only_deposit(ctx: Context<AuthOnlyDeposit>, amount: u64) -> Result<()> {
        let raw = &ctx.accounts.raw_vault;
        **raw.try_borrow_mut_lamports()? += amount;
        msg!("auth_only_deposit 完了");
        Ok(())
    }

    /// パターンC：手動所有者チェック（has_one は使わずフィールド比較）
    pub fn manual_owner_deposit(ctx: Context<ManualOwnerDeposit>, amount: u64) -> Result<()> {
        let vault = &ctx.accounts.managed_vault;
        if vault.owner == ctx.accounts.user.key() {
            let info = vault.to_account_info();
            **info.try_borrow_mut_lamports()? += amount;
        }
        msg!("manual_owner_deposit 完了");
        Ok(())
    }

    /// パターンD：remaining_accounts に含まれる全アカウントをまとめて更新
    pub fn batch_deposit(ctx: Context<BatchDeposit>, amount: u64) -> Result<()> {
        for acct in ctx.remaining_accounts.iter() {
            // 簡易チェックだけ行い、更新
            if acct.lamports() >= 0 {
                **acct.try_borrow_mut_lamports()? += amount;
            }
        }
        msg!("batch_deposit 完了");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct EnforcedDeposit<'info> {
    /// has_one で Vault.owner == owner を検証
    #[account(mut, has_one = owner)]
    pub vault_acc: Account<'info, Vault>,
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AuthOnlyDeposit<'info> {
    /// CHECK: raw AccountInfo、所有権チェックなし
    #[account(mut)]
    pub raw_vault: AccountInfo<'info>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ManualOwnerDeposit<'info> {
    /// mut 指定のみ、has_one は使わず手動チェック
    #[account(mut)]
    pub managed_vault: Account<'info, Vault>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BatchDeposit<'info> {
    /// mut 指定のみ、remaining_accounts で追加アカウントを受け取る
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
    // remaining_accounts に任意の追加アカウント
}

#[account]
pub struct Vault {
    /// 正当なオーナーの Pubkey
    pub owner: Pubkey,
}
