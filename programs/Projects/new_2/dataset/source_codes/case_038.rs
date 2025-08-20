use anchor_lang::prelude::*;
use std::cmp::Ordering;

declare_id!("ProgFlow024IDUNIQUECASE024XXX");

#[program]
pub mod case_024_flow {
    use super::*;

    /// モード1：has_one + signer で完全ガード
    pub fn ordered_deposit(ctx: Context<OrderedDeposit>, amount: u64) -> Result<()> {
        let ai = ctx.accounts.secured_vault.to_account_info();
        let before = ai.lamports();
        **ai.try_borrow_mut_lamports()? = before + amount;
        msg!("ordered_deposit: {}→{}", before, before + amount);
        Ok(())
    }

    /// モード2：署名のみ検証、while ループで複数回加算（owner 未検証）
    pub fn iterated_deposit(ctx: Context<IteratedDeposit>, amount: u64) -> Result<()> {
        let raw = &ctx.accounts.unchecked_vault;
        let mut count = 0;
        while count < 2 {
            **raw.try_borrow_mut_lamports()? += amount;
            count += 1;
        }
        msg!("iterated_deposit 完了");
        Ok(())
    }

    /// モード3：match で手動オーナーチェック（if に “=” を使わず）
    pub fn matched_deposit(ctx: Context<MatchedDeposit>, amount: u64) -> Result<()> {
        let vault = &ctx.accounts.vault_data;
        let caller = ctx.accounts.caller.key();
        match vault.owner.cmp(&caller) {
            Ordering::Equal => {
                let ai = vault.to_account_info();
                let prev = ai.lamports();
                **ai.try_borrow_mut_lamports()? = prev + amount;
                msg!("matched_deposit 適用: {}", prev + amount);
            }
            _ => msg!("matched_deposit: 権限なし"),
        }
        Ok(())
    }

    /// モード4：remaining_accounts に含まれる全アカウントをまとめて更新
    pub fn batch_deposit(ctx: Context<BatchDeposit>, amount: u64) -> Result<()> {
        for acct in ctx.remaining_accounts.iter() {
            let before = acct.lamports();
            **acct.try_borrow_mut_lamports()? = before + amount;
        }
        msg!("batch_deposit 完了");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct OrderedDeposit<'info> {
    #[account(mut, has_one = authority)]
    pub secured_vault: Account<'info, Vault>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct IteratedDeposit<'info> {
    /// CHECK: raw の AccountInfo、所有者チェック省略
    #[account(mut)]
    pub unchecked_vault: AccountInfo<'info>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MatchedDeposit<'info> {
    #[account(mut)]
    pub vault_data: Account<'info, Vault>,
    /// CHECK: 比較用の実行者キー
    pub caller: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BatchDeposit<'info> {
    pub system_program: Program<'info, System>,
    // remaining_accounts に任意の追加アカウントを受け取る
}

#[account]
pub struct Vault {
    pub owner: Pubkey,
}
