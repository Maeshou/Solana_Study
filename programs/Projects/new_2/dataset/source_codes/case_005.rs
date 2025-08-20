use anchor_lang::prelude::*;

declare_id!("ProgMixedOwner008IDX7Y3XP9ABCDEFFEDCBA123456");

#[program]
pub mod case_008_diverse {
    use super::*;

    /// パターン1：完全検証（has_one + signer）で直接増加
    pub fn secure_increment(ctx: Context<SecureIncrementCtx>, amount: u64) -> Result<()> {
        let acct_info = ctx.accounts.secure_vault.to_account_info();
        let current = **acct_info.try_borrow_lamports()?;
        **acct_info.try_borrow_mut_lamports()? = current + amount;
        msg!("secure_increment 完了");
        Ok(())
    }

    /// パターン2：署名のみ検証、単一ループで増加（オーナーチェックなし）
    pub fn loop_increment(ctx: Context<SignedLoopIncrementCtx>, amount: u64) -> Result<()> {
        for _ in 0..1 {
            **ctx.accounts.partial_vault.try_borrow_mut_lamports()? += amount;
        }
        msg!("loop_increment 完了");
        Ok(())
    }

    /// パターン3：has_one のみ検証、クロージャ内で増加（署名不要）
    pub fn closure_increment(ctx: Context<OwnerClosureIncrementCtx>, amount: u64) -> Result<()> {
        let mut info = ctx.accounts.owner_vault.to_account_info();
        (|| -> Result<()> {
            **info.try_borrow_mut_lamports()? += amount;
            Ok(())
        })()?;
        msg!("closure_increment 完了");
        Ok(())
    }

    /// パターン4：未検証 raw、直接加算
    pub fn raw_increment(ctx: Context<OpenIncrementCtx>, amount: u64) -> Result<()> {
        **ctx.accounts.any_vault.try_borrow_mut_lamports()? += amount;
        msg!("raw_increment 完了");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SecureIncrementCtx<'info> {
    /// has_one で Vault.owner == authority を検証
    #[account(mut, has_one = authority)]
    pub secure_vault: Account<'info, Vault>,
    /// 署名チェック
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SignedLoopIncrementCtx<'info> {
    /// raw AccountInfo、owner チェックなし
    pub partial_vault: AccountInfo<'info>,
    /// 署名チェック
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct OwnerClosureIncrementCtx<'info> {
    /// has_one で Vault.owner == authority を検証（署名不要）
    #[account(mut, has_one = authority)]
    pub owner_vault: Account<'info, Vault>,
    /// AccountInfo のみ
    pub authority: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct OpenIncrementCtx<'info> {
    /// raw AccountInfo、全く検証なし
    pub any_vault: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault {
    /// 正当なオーナーの Pubkey
    pub authority: Pubkey,
}
