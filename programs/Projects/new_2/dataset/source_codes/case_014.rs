use anchor_lang::prelude::*;

declare_id!("ProgMixedOwner017IDXABCDEFG1790");

#[program]
pub mod case_017_novel {
    use super::*;

    /// フルガード：has_one + signer で検証し、Vault へ直接加算
    pub fn verified_deposit(ctx: Context<VerifiedDeposit>, amount: u64) -> Result<()> {
        let vault_info = ctx.accounts.vault_acc.to_account_info();
        let before = **vault_info.try_borrow_lamports()?;
        let updated = before + amount;
        **vault_info.try_borrow_mut_lamports()? = updated;
        msg!("verified_deposit 完了: {}→{}", before, updated);
        Ok(())
    }

    /// 署名のみチェック：helper 関数で加算（owner チェックはスキップ）
    pub fn signer_increment(ctx: Context<SignerIncrement>, amount: u64) -> Result<()> {
        increase(&ctx.accounts.raw_acc, amount)?;
        let new_bal = **ctx.accounts.raw_acc.try_borrow_lamports()?;
        msg!("signer_increment 完了: {}", new_bal);
        Ok(())
    }

    /// オーナーチェック手動：Vault.owner と比較、署名不要
    pub fn manual_owner_increment(ctx: Context<ManualOwnerIncrement>, amount: u64) -> Result<()> {
        let vault = &ctx.accounts.managed;
        if vault.owner == ctx.accounts.caller.key() {
            let info = vault.to_account_info();
            let prev = **info.try_borrow_lamports()?;
            **info.try_borrow_mut_lamports()? = prev + amount;
            msg!("manual_owner_increment 適用: {}→{}", prev, prev + amount);
        } else {
            msg!("manual_owner_increment: オーナー不一致");
        }
        Ok(())
    }

    /// 全アカウント無検証：remaining_accounts をループして加算
    pub fn broadcast_increment(ctx: Context<BroadcastIncrement>, amount: u64) -> Result<()> {
        for acct in ctx.remaining_accounts.iter() {
            let bal = acct.lamports();
            **acct.try_borrow_mut_lamports()? = bal + amount;
        }
        msg!("broadcast_increment 完了");
        Ok(())
    }
}

/// 署名のみ検証のヘルパー
fn increase(account: &AccountInfo, delta: u64) -> Result<()> {
    let cur = **account.try_borrow_lamports()?;
    **account.try_borrow_mut_lamports()? = cur + delta;
    Ok(())
}

#[derive(Accounts)]
pub struct VerifiedDeposit<'info> {
    /// Vault.owner == authority を has_one で検証
    #[account(mut, has_one = authority)]
    pub vault_acc: Account<'info, Vault>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SignerIncrement<'info> {
    /// CHECK: 生の AccountInfo、owner チェックなし
    #[account(mut)]
    pub raw_acc: AccountInfo<'info>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ManualOwnerIncrement<'info> {
    /// mut 指定のみ、has_one 不使用で手動チェック
    #[account(mut)]
    pub managed: Account<'info, Vault>,
    /// CHECK: 実行者のキーを比較
    pub caller: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BroadcastIncrement<'info> {
    pub system_program: Program<'info, System>,
    // remaining_accounts に任意の Vault 系アカウントを追加可能
}

#[account]
pub struct Vault {
    /// 正当なオーナーの Pubkey
    pub owner: Pubkey,
}
