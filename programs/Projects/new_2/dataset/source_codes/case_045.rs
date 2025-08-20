use anchor_lang::prelude::*;

declare_id!("ProgMixedOwner028IDNEWVAR00028");

#[program]
pub mod case_028_simple {
    use super::*;

    /// 完全検証：has_one + signer で所有権と署名を確認
    pub fn full_insert(ctx: Context<FullInsertCtx>, amount: u64) -> Result<()> {
        let account = ctx.accounts.checked_account.to_account_info();
        let current = **account.try_borrow_lamports()?;
        **account.try_borrow_mut_lamports()? = current + amount;
        msg!("full_insert complete");
        Ok(())
    }

    /// 署名のみ検証：raw AccountInfo を直接操作（owner チェックなし）
    pub fn signer_insert(ctx: Context<SignerInsertCtx>, amount: u64) -> Result<()> {
        let account = &ctx.accounts.raw_account;
        let mut balance = **account.try_borrow_lamports()?;
        balance += amount;
        **account.try_borrow_mut_lamports()? = balance;
        msg!("signer_insert complete");
        Ok(())
    }

    /// 手動オーナーチェック：if 文で owner フィールドを比較
    pub fn owner_check_insert(ctx: Context<OwnerCheckInsertCtx>, amount: u64) -> Result<()> {
        if ctx.accounts.vault.owner == ctx.accounts.user.key() {
            let account = ctx.accounts.vault.to_account_info();
            let current = **account.try_borrow_lamports()?;
            **account.try_borrow_mut_lamports()? = current + amount;
        }
        msg!("owner_check_insert processed");
        Ok(())
    }

    /// 未検証：誰でも操作可能な AccountInfo
    pub fn open_insert(ctx: Context<OpenInsertCtx>, amount: u64) -> Result<()> {
        let account = &ctx.accounts.free_account;
        let before = **account.try_borrow_lamports()?;
        **account.try_borrow_mut_lamports()? = before + amount;
        msg!("open_insert complete");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct FullInsertCtx<'info> {
    #[account(mut, has_one = authority)]
    pub checked_account: Account<'info, Vault>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SignerInsertCtx<'info> {
    /// CHECK: raw AccountInfo、所有権チェックなし
    #[account(mut)]
    pub raw_account: AccountInfo<'info>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct OwnerCheckInsertCtx<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    /// CHECK: 手動チェック用の実行者キー
    pub user: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct OpenInsertCtx<'info> {
    /// CHECK: 完全未検証の AccountInfo
    #[account(mut)]
    pub free_account: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault {
    pub owner: Pubkey,
}
