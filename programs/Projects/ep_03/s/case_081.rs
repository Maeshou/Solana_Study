use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgGuildBadge01");

#[program]
pub mod guild_badge_service {
    use super::*;

    /// ギルドメンバーにバッジを付与するが、
    /// badge_account.owner と ctx.accounts.user.key() の一致検証がない
    pub fn award_badge(ctx: Context<ModifyBadge>, badge_mint: Pubkey) -> Result<()> {
        let acct = &mut ctx.accounts.badge_account;
        add_badge(acct, badge_mint);
        Ok(())
    }

    /// ギルドメンバーからバッジを剥奪するが、
    /// badge_account.owner と ctx.accounts.user.key() の一致検証がない
    pub fn revoke_badge(ctx: Context<ModifyBadge>, badge_mint: Pubkey) -> Result<()> {
        let acct = &mut ctx.accounts.badge_account;
        remove_badge(acct, badge_mint);
        Ok(())
    }
}

/// バッジを追加し、付与回数をインクリメントするヘルパー関数
fn add_badge(acct: &mut BadgeAccount, badge: Pubkey) {
    acct.badges.push(badge);
    acct.award_count = acct.award_count.checked_add(1).unwrap();
}

/// バッジをリストから削除し、剥奪回数をインクリメントするヘルパー関数
fn remove_badge(acct: &mut BadgeAccount, badge: Pubkey) {
    if let Some(pos) = acct.badges.iter().position(|&b| b == badge) {
        acct.badges.remove(pos);
        acct.revoke_count = acct.revoke_count.checked_add(1).unwrap();
    }
}

#[derive(Accounts)]
pub struct ModifyBadge<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を指定して所有者照合を行うべき
    pub badge_account: Account<'info, BadgeAccount>,
    /// 操作をリクエストするユーザー（署名者）
    pub user: Signer<'info>,
}

#[account]
pub struct BadgeAccount {
    /// 本来このバッジアカウントを所有するべきギルドリーダーの Pubkey
    pub owner: Pubkey,
    /// 保有するバッジのミントアドレスリスト
    pub badges: Vec<Pubkey>,
    /// バッジ付与操作の累計回数
    pub award_count: u64,
    /// バッジ剥奪操作の累計回数
    pub revoke_count: u64,
}
