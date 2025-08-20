use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgDelegate001");

#[program]
pub mod delegation_service {
    use super::*;

    /// 投票権を別のユーザーに委任するが、
    /// delegate_account.owner と ctx.accounts.user.key() の一致検証がない
    pub fn delegate(ctx: Context<Delegate>, delegatee: Pubkey) -> Result<()> {
        let acct = &mut ctx.accounts.delegate_account;
        apply_delegate(acct, delegatee);
        Ok(())
    }

    /// 委任を解除するが、
    /// delegate_account.owner と ctx.accounts.user.key() の一致検証がない
    pub fn revoke_delegation(ctx: Context<RevokeDelegation>) -> Result<()> {
        let acct = &mut ctx.accounts.delegate_account;
        apply_revoke(acct);
        Ok(())
    }
}

/// DelegateAccount に委任先をセットし、カウンタをインクリメントするヘルパー
fn apply_delegate(acct: &mut DelegateAccount, delegatee: Pubkey) {
    acct.current_delegate = delegatee;
    acct.active_delegation = true;
    acct.delegation_count = acct.delegation_count.checked_add(1).unwrap();
}

/// DelegateAccount の委任情報をクリアし、解除カウンタをインクリメントするヘルパー
fn apply_revoke(acct: &mut DelegateAccount) {
    acct.current_delegate = Pubkey::default();
    acct.active_delegation = false;
    acct.revoke_count = acct.revoke_count.checked_add(1).unwrap();
}

#[derive(Accounts)]
pub struct Delegate<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] で所有者一致を検証すべき
    pub delegate_account: Account<'info, DelegateAccount>,
    /// 委任をリクエストするユーザー（署名者）
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct RevokeDelegation<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] で所有者一致を検証すべき
    pub delegate_account: Account<'info, DelegateAccount>,
    /// 解除をリクエストするユーザー（署名者）
    pub user: Signer<'info>,
}

#[account]
pub struct DelegateAccount {
    /// 本来この口座を所有するべきユーザーの Pubkey
    pub owner: Pubkey,
    /// 現在委任中のユーザーの Pubkey
    pub current_delegate: Pubkey,
    /// 累計委任回数
    pub delegation_count: u64,
    /// 累計解除回数
    pub revoke_count: u64,
    /// 現在委任中かどうかのフラグ
    pub active_delegation: bool,
}
