use anchor_lang::prelude::*;
use std::mem;

declare_id!("ProgVault031IDXDISTINCTCASE031");

#[program]
pub mod case_031_distinct {
    use super::*;

    /// パターン1：has_one + signer で saturating_add を使った安全加算
    pub fn safe_add(ctx: Context<SafeAddCtx>, amount: u64) -> Result<()> {
        let acct = ctx.accounts.vault.to_account_info();
        let before = acct.lamports();
        **acct.try_borrow_mut_lamports()? = before.saturating_add(amount);
        msg!("safe_add: {}→{}", before, before.saturating_add(amount));
        Ok(())
    }

    /// パターン2：mem::replace を使ってバランスを置き換え（ownerチェックなし、署名のみ）
    pub fn replace_add(ctx: Context<ReplaceAddCtx>, amount: u64) -> Result<()> {
        let info = &ctx.accounts.raw_vault;
        let mut balance = **info.try_borrow_lamports()?;
        let old = mem::replace(&mut balance, balance + amount);
        **info.try_borrow_mut_lamports()? = balance;
        msg!("replace_add: {}→{}", old, balance);
        Ok(())
    }

    /// パターン3：オーナー不正時に早期リターン（manual owner check）
    pub fn owner_guard(ctx: Context<OwnerGuardCtx>, amount: u64) -> Result<()> {
        if ctx.accounts.vault.owner != ctx.accounts.user.key() {
            msg!("owner_guard: unauthorized");
            return Ok(());
        }
        let acct = ctx.accounts.vault.to_account_info();
        let prev = acct.lamports();
        **acct.try_borrow_mut_lamports()? = prev + amount;
        msg!("owner_guard applied: {}→{}", prev, prev + amount);
        Ok(())
    }

    /// パターン4：primary vault にフル、残りアカウントには半額だけ加算
    pub fn split_add(ctx: Context<SplitAddCtx>, amount: u64) -> Result<()> {
        // primary vault gets full amount
        let primary_info = ctx.accounts.primary.to_account_info();
        **primary_info.try_borrow_mut_lamports()? += amount;
        // remaining_accounts get half
        let bonus = amount / 2;
        for acct in &ctx.remaining_accounts {
            **acct.try_borrow_mut_lamports()? += bonus;
        }
        msg!("split_add done");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SafeAddCtx<'info> {
    #[account(mut, has_one = authority)]
    pub vault: Account<'info, Vault>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ReplaceAddCtx<'info> {
    /// CHECK: raw AccountInfo、所有権未検証
    #[account(mut)]
    pub raw_vault: AccountInfo<'info>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct OwnerGuardCtx<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SplitAddCtx<'info> {
    #[account(mut)]
    pub primary: Account<'info, Vault>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    // remaining_accounts に任意の追加アカウントを受け取る
}

#[account]
pub struct Vault {
    /// 正当なオーナーの Pubkey
    pub owner: Pubkey,
}
