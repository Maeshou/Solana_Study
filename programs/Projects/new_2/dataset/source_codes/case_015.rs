use anchor_lang::prelude::*;

declare_id!("ProgMixedOwner018IDXEXOTIC87654321");

#[program]
pub mod case_018_exotic {
    use super::*;

    /// パターンA：has_one + signer でオーナー検証済み
    pub fn guarded_deposit(ctx: Context<GuardedDeposit>, amount: u64) -> Result<()> {
        let vault_info = ctx.accounts.secure_vault.to_account_info();
        let old_balance = **vault_info.try_borrow_lamports()?;
        let new_balance = old_balance + amount;
        **vault_info.try_borrow_mut_lamports()? = new_balance;
        msg!("guarded_deposit: {}→{}", old_balance, new_balance);
        Ok(())
    }

    /// パターンB：署名のみチェック（owner チェックなし）
    pub fn signed_transfer(ctx: Context<SignedTransfer>, amount: u64) -> Result<()> {
        let raw = &ctx.accounts.raw_vault;
        let prev = **raw.try_borrow_lamports()?;
        **raw.try_borrow_mut_lamports()? = prev + amount;
        msg!("signed_transfer: {}", **raw.try_borrow_lamports()?);
        Ok(())
    }

    /// パターンC：手動オーナーチェック（has_one 不使用、if で比較）
    pub fn owner_only_transfer(ctx: Context<OwnerOnlyTransfer>, amount: u64) -> Result<()> {
        let vault = &ctx.accounts.vault_acc;
        let caller = ctx.accounts.user.key();
        if vault.owner == caller {
            let info = vault.to_account_info();
            let before = **info.try_borrow_lamports()?;
            **info.try_borrow_mut_lamports()? = before + amount;
            msg!("owner_only_transfer: {}", before + amount);
        }
        Ok(())
    }

    /// パターンD：remaining_accounts に含まれる全アカウントをまとめて更新
    pub fn mass_credit(ctx: Context<MassCredit>, amount: u64) -> Result<()> {
        for acct in ctx.remaining_accounts.iter() {
            let bal = acct.lamports();
            **acct.try_borrow_mut_lamports()? = bal + amount;
        }
        msg!("mass_credit 完了");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct GuardedDeposit<'info> {
    #[account(mut, has_one = owner)]
    pub secure_vault: Account<'info, Vault>,
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SignedTransfer<'info> {
    #[account(mut)]
    pub raw_vault: AccountInfo<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct OwnerOnlyTransfer<'info> {
    #[account(mut)]
    pub vault_acc: Account<'info, Vault>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MassCredit<'info> {
    pub system_program: Program<'info, System>,
    // remaining_accounts に任意のアカウントを受け取る
}

#[account]
pub struct Vault {
    pub owner: Pubkey,
}
