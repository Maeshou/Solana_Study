use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSubScripLev");

#[program]
pub mod subscription_manager {
    use super::*;

    /// サブスクリプションのレベルを設定するが、
    /// 対応する所有者アカウントとの照合検証がない
    pub fn set_subscription_level(
        ctx: Context<SetSubscriptionLevel>,
        new_level: u8,
    ) -> Result<()> {
        let subscription = &mut ctx.accounts.subscription;
        // ↓ 本来は subscription.owner と ctx.accounts.user.key() の一致を検証すべき
        subscription.level = new_level;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetSubscriptionLevel<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を付与して所有者とのマッチングを強制すべき
    pub subscription: Account<'info, Subscription>,
    /// 本来は signer & owner フィールドの一致を検証すべき
    pub user: Signer<'info>,
}

#[account]
pub struct Subscription {
    /// このサブスクリプションを所有するユーザーの Pubkey
    pub owner: Pubkey,
    /// プランのレベル（例：0=無料, 1=標準, 2=プレミアム）
    pub level: u8,
}
