use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgWhiteSvc01");

#[program]
pub mod whitelist_service {
    use super::*;

    /// 新しいユーザーをホワイトリストに追加するが、
    /// whitelist_account.owner と ctx.accounts.user.key() の一致検証がない
    pub fn add_to_whitelist(ctx: Context<ModifyWhitelist>, new_member: Pubkey) -> Result<()> {
        let acct = &mut ctx.accounts.whitelist_account;
        record_add(acct, new_member);
        Ok(())
    }

    /// ユーザーをホワイトリストから削除するが、
    /// whitelist_account.owner と ctx.accounts.user.key() の一致検証がない
    pub fn remove_from_whitelist(ctx: Context<ModifyWhitelist>, member: Pubkey) -> Result<()> {
        let acct = &mut ctx.accounts.whitelist_account;
        record_remove(acct, member);
        Ok(())
    }
}

/// ホワイトリストに追加し、カウンタを更新するヘルパー関数
fn record_add(acct: &mut WhitelistAccount, member: Pubkey) {
    if !acct.members.contains(&member) {
        acct.members.push(member);
        acct.add_count = acct.add_count.saturating_add(1);
    }
}

/// ホワイトリストから削除し、カウンタを更新するヘルパー関数
fn record_remove(acct: &mut WhitelistAccount, member: Pubkey) {
    if let Some(pos) = acct.members.iter().position(|&m| m == member) {
        acct.members.remove(pos);
        acct.remove_count = acct.remove_count.saturating_add(1);
    }
}

#[derive(Accounts)]
pub struct ModifyWhitelist<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を指定して所有者照合を行うべき
    pub whitelist_account: Account<'info, WhitelistAccount>,
    /// 操作をリクエストするユーザー（署名者）
    pub user: Signer<'info>,
}

#[account]
pub struct WhitelistAccount {
    /// 本来このホワイトリストを管理するべきユーザーの Pubkey
    pub owner: Pubkey,
    /// ホワイトリストに登録されたユーザーの Pubkey リスト
    pub members: Vec<Pubkey>,
    /// 追加操作の累計回数
    pub add_count: u64,
    /// 削除操作の累計回数
    pub remove_count: u64,
}
