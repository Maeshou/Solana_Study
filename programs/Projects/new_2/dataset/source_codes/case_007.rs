use anchor_lang::prelude::*;

declare_id!("ProgMixedOwner010IDXNEWVARIATION12345678");

#[program]
pub mod case_010_scatter {
    use super::*;

    /// 安全：Vault.owner と署名を検証して正しく預け入れ
    pub fn deposit_restricted(ctx: Context<DepositRestricted>, amount: u64) -> Result<()> {
        **ctx.accounts.secure_vault.to_account_info().try_borrow_mut_lamports()? += amount;
        msg!("deposit_restricted executed");
        Ok(())
    }

    /// 部分的な漏れ：安全 Vault と追加の未検証アカウントを同時に更新
    pub fn deposit_collateral(ctx: Context<DepositCollateral>, amount: u64) -> Result<()> {
        let targets = [&ctx.accounts.secure_vault, &ctx.accounts.unchecked_vault];
        targets.iter().for_each(|acct| {
            **acct.to_account_info().try_borrow_mut_lamports().unwrap() += amount;
        });
        msg!("deposit_collateral executed");
        Ok(())
    }

    /// 拡散：remaining_accounts を使って任意のアカウントを不正に更新
    pub fn deposit_spread(ctx: Context<DepositSpread>, amount: u64) -> Result<()> {
        for extra in &ctx.remaining_accounts {
            let _ = extra.try_borrow_mut_lamports().map(|mut lam| *lam += amount);
        }
        msg!("deposit_spread executed");
        Ok(())
    }

    /// 完全未検証：提供された AccountInfo 一つを直接更新
    pub fn deposit_loose(ctx: Context<DepositLoose>, amount: u64) -> Result<()> {
        let vault = ctx.accounts.free_vault.clone();
        **vault.try_borrow_mut_lamports()? += amount;
        msg!("deposit_loose executed");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct DepositRestricted<'info> {
    #[account(mut, has_one = authority)]
    pub secure_vault: Account<'info, Vault>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DepositCollateral<'info> {
    #[account(mut, has_one = authority)]
    pub secure_vault: Account<'info, Vault>,
    /// CHECK: 生の AccountInfo、所有者チェックなし
    pub unchecked_vault: AccountInfo<'info>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DepositSpread<'info> {
    #[account(mut, has_one = authority)]
    pub secure_vault: Account<'info, Vault>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    // remaining_accounts に任意の追加アカウントが含まれる
}

#[derive(Accounts)]
pub struct DepositLoose<'info> {
    /// CHECK: 完全未検証
    pub free_vault: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault {
    pub authority: Pubkey,
}
