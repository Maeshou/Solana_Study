use anchor_lang::prelude::*;

declare_id!("ProgMixedOwner029IDXFRESHNEWCASE029");

#[program]
pub mod case_029_alternate {
    use super::*;

    /// モードA：アカウントメソッド呼び出しで安全に増加（has_one + signer）
    pub fn method_push(ctx: Context<MethodPush>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.verified_vault;
        vault.push(amount)?;
        msg!("method_push 完了");
        Ok(())
    }

    /// モードB：生 AccountInfo に直接加算（署名のみチェック、owner は未検証）
    pub fn raw_push(ctx: Context<RawPush>, amount: u64) -> Result<()> {
        let info = &ctx.accounts.raw_vault;
        let before = **info.try_borrow_lamports()?;
        **info.try_borrow_mut_lamports()? = before + amount;
        msg!("raw_push: {}→{}", before, before + amount);
        Ok(())
    }

    /// モードC：手動オーナーチェック後に更新（署名不要）
    pub fn manual_push(ctx: Context<ManualPush>, amount: u64) -> Result<()> {
        let vault = &ctx.accounts.data_vault;
        if vault.owner == ctx.accounts.requester.key() {
            let info = vault.to_account_info();
            let old = **info.try_borrow_lamports()?;
            **info.try_borrow_mut_lamports()? = old + amount;
            msg!("manual_push 適用: {}→{}", old, old + amount);
        }
        Ok(())
    }

    /// モードD：remaining_accounts 全件をまとめて更新（完全未検証）
    pub fn any_push(ctx: Context<AnyPush>, amount: u64) -> Result<()> {
        for acct in &ctx.remaining_accounts {
            let bal = acct.lamports();
            **acct.try_borrow_mut_lamports()? = bal + amount;
        }
        msg!("any_push 完了");
        Ok(())
    }
}

impl Vault {
    /// ヘルパー：アカウント構造体内で lamports を更新
    pub fn push(&self, delta: u64) -> Result<()> {
        let info = self.to_account_info();
        let before = **info.try_borrow_lamports()?;
        **info.try_borrow_mut_lamports()? = before + delta;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MethodPush<'info> {
    #[account(mut, has_one = authority)]
    pub verified_vault: Account<'info, Vault>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RawPush<'info> {
    #[account(mut)]
    pub raw_vault: AccountInfo<'info>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ManualPush<'info> {
    #[account(mut)]
    pub data_vault: Account<'info, Vault>,
    /// CHECK: オーナー比較用
    pub requester: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AnyPush<'info> {
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    // remaining_accounts に任意の AccountInfo が含まれる
}

#[account]
pub struct Vault {
    pub owner: Pubkey,
}
