use anchor_lang::prelude::*;

declare_id!("ProgMixedOwner020IDXZXY9876543210AB");

#[program]
pub mod case_020_spectrum {
    use super::*;

    /// Case A: Anchor の has_one + signer で完全保護
    pub fn full_add(ctx: Context<FullAddCtx>, amount: u64) -> Result<()> {
        let vault_info = ctx.accounts.safeguard.to_account_info();
        let original = **vault_info.try_borrow_lamports()?;
        let result = original + amount;
        **vault_info.try_borrow_mut_lamports()? = result;
        msg!("full_add: {} → {}", original, result);
        Ok(())
    }

    /// Case B: 署名のみチェック、タプル展開で AccountInfo を操作（所有者未検証）
    pub fn only_sign(ctx: Context<OnlySignCtx>, amount: u64) -> Result<()> {
        let OnlySignCtx { raw_account, signer: _, .. } = &ctx.accounts;
        let before = **raw_account.try_borrow_lamports()?;
        **raw_account.try_borrow_mut_lamports()? = before + amount;
        msg!("only_sign: {}", before + amount);
        Ok(())
    }

    /// Case C: 手動で owner フィールドを照合、Signer は利用せず
    pub fn manual_test(ctx: Context<ManualTestCtx>, amount: u64) -> Result<()> {
        let vault = &ctx.accounts.vault_account;
        let caller = &ctx.accounts.user_key;
        if vault.owner == caller.key() {
            let info = vault.to_account_info();
            let prev = **info.try_borrow_lamports()?;
            **info.try_borrow_mut_lamports()? = prev + amount;
            msg!("manual_test applied: {}", prev + amount);
        }
        Ok(())
    }

    /// Case D: remaining_accounts に含まれる全アカウントを一括更新
    pub fn bulk(ctx: Context<BulkCtx>, amount: u64) -> Result<()> {
        for acc in ctx.remaining_accounts.iter() {
            let before = acc.lamports();
            **acc.try_borrow_mut_lamports()? = before + amount;
        }
        msg!("bulk complete");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct FullAddCtx<'info> {
    #[account(mut, has_one = authority)]
    pub safeguard: Account<'info, Vault>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct OnlySignCtx<'info> {
    #[account(mut)]
    pub raw_account: AccountInfo<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ManualTestCtx<'info> {
    #[account(mut)]
    pub vault_account: Account<'info, Vault>,
    /// CHECK: 比較のための署名者
    pub user_key: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BulkCtx<'info> {
    pub system_program: Program<'info, System>,
    // remaining_accounts に任意のアカウントを受け取る
}

#[account]
pub struct Vault {
    pub owner: Pubkey,
}
