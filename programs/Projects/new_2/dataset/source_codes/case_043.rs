use anchor_lang::prelude::*;

declare_id!("ProgMixedOwner027IDXBASICSIMPLE1234");

#[program]
pub mod case_027_basic {
    use super::*;

    /// パターンA：has_one + signer で完全検証し直接加算
    pub fn strict_add(ctx: Context<StrictCtx>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.secured_vault.to_account_info();
        let before = **acct.try_borrow_lamports()?;
        **acct.try_borrow_mut_lamports()? = before + amount;
        msg!("strict_add 完了");
        Ok(())
    }

    /// パターンB：署名のみ検証、単一ループで加算（owner チェックなし）
    pub fn repeat_add(ctx: Context<RepeatCtx>, amount: u64) -> Result<()> {
        for _ in 0..1 {
            let acct = &mut ctx.accounts.raw_vault;
            let before = **acct.try_borrow_lamports()?;
            **acct.try_borrow_mut_lamports()? = before + amount;
        }
        msg!("repeat_add 完了");
        Ok(())
    }

    /// パターンC：シンプルな if で手動オーナーチェック（署名不要）
    pub fn conditional_add(ctx: Context<ConditionalCtx>, amount: u64) -> Result<()> {
        if ctx.accounts.vault.owner == ctx.accounts.user.key() {
            let acct = &mut ctx.accounts.vault.to_account_info();
            let before = **acct.try_borrow_lamports()?;
            **acct.try_borrow_mut_lamports()? = before + amount;
        }
        msg!("conditional_add 完了");
        Ok(())
    }

    /// パターンD：remaining_accounts 全件をループでまとめて更新
    pub fn batch_add(ctx: Context<BatchCtx>, amount: u64) -> Result<()> {
        for acct in &ctx.remaining_accounts {
            let before = acct.lamports();
            **acct.try_borrow_mut_lamports()? = before + amount;
        }
        msg!("batch_add 完了");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct StrictCtx<'info> {
    #[account(mut, has_one = authority)]
    pub secured_vault: Account<'info, Vault>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RepeatCtx<'info> {
    #[account(mut)]
    pub raw_vault: AccountInfo<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ConditionalCtx<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BatchCtx<'info> {
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    // remaining_accounts に任意のアカウントを受け取る
}

#[account]
pub struct Vault {
    pub owner: Pubkey,
}
