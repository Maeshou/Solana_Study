use anchor_lang::prelude::*;

declare_id!("ProgMixedOwner023IDXUNIQUEXYZ789");

#[program]
pub mod case_023_unified {
    use super::*;

    /// フルガード：has_one + signer で所有権と署名を検証
    pub fn withdraw_protected(ctx: Context<ProtectedCtx>, amount: u64) -> Result<()> {
        let vault_info = ctx.accounts.vault_account.to_account_info();
        let initial = **vault_info.try_borrow_lamports()?;
        **vault_info.try_borrow_mut_lamports()? = initial.saturating_sub(amount);
        msg!("withdraw_protected: {}→{}", initial, initial - amount);
        Ok(())
    }

    /// 手動チェック：assert_eq! マクロで vault.owner と caller を検証
    pub fn withdraw_assert(ctx: Context<AssertCtx>, amount: u64) -> Result<()> {
        let vault_data = &ctx.accounts.vault_data;
        assert_eq!(vault_data.owner, ctx.accounts.user.key());
        let info = vault_data.to_account_info();
        let before = **info.try_borrow_lamports()?;
        **info.try_borrow_mut_lamports()? = before.saturating_sub(amount);
        msg!("withdraw_assert: {}", before - amount);
        Ok(())
    }

    /// 署名のみ：Signer 指定あり、所有権チェックは行わない
    pub fn withdraw_signonly(ctx: Context<SignedCtx>, amount: u64) -> Result<()> {
        let raw = &ctx.accounts.raw_vault;
        let current = **raw.try_borrow_lamports()?;
        **raw.try_borrow_mut_lamports()? = current.saturating_sub(amount);
        msg!("withdraw_signonly: {}", current - amount);
        Ok(())
    }

    /// 未検証：任意の追加アカウントにアクセス可能
    pub fn withdraw_generic(ctx: Context<GenericCtx>, amount: u64) -> Result<()> {
        for acct in ctx.remaining_accounts.iter() {
            let bal = acct.lamports();
            **acct.try_borrow_mut_lamports()? = bal.saturating_sub(amount);
        }
        msg!("withdraw_generic completed");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ProtectedCtx<'info> {
    #[account(mut, has_one = authority)]
    pub vault_account: Account<'info, Vault>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AssertCtx<'info> {
    #[account(mut)]
    pub vault_data: Account<'info, Vault>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SignedCtx<'info> {
    #[account(mut)]
    pub raw_vault: AccountInfo<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GenericCtx<'info> {
    pub system_program: Program<'info, System>,
    // remaining_accounts に任意の AccountInfo を受け取る
}

#[account]
pub struct Vault {
    pub owner: Pubkey,
}
