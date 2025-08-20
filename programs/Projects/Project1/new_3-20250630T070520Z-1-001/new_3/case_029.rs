use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgAvatarSv01");

#[program]
pub mod avatar_customization {
    use super::*;

    /// ユーザーのプロフィールにアバターNFTを設定するが、
    /// profile_account.owner と ctx.accounts.user.key() の照合検証がない
    pub fn set_avatar(ctx: Context<SetAvatar>, avatar_mint: Pubkey) -> Result<()> {
        let profile = &mut ctx.accounts.profile_account;
        // ↓ 本来は profile.owner と ctx.accounts.user.key() の一致を検証すべき
        profile.avatar = avatar_mint;
        // 更新回数をインクリメント
        profile.update_count = profile.update_count.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetAvatar<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を指定して所有者照合を行うべき
    pub profile_account: Account<'info, ProfileAccount>,
    /// アバターを設定するユーザー（署名者）
    pub user: Signer<'info>,
}

#[account]
pub struct ProfileAccount {
    /// このプロフィールを所有するユーザーの Pubkey
    pub owner: Pubkey,
    /// 設定中のアバターNFTミントアドレス
    pub avatar: Pubkey,
    /// アバター更新回数
    pub update_count: u64,
}
