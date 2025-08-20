use anchor_lang::prelude::*;

declare_id!("MixMorA2333333333333333333333333333333333");

#[program]
pub mod mixed_more3 {
    pub fn change_avatar(
        ctx: Context<Change>,
        uri: String,
    ) -> Result<()> {
        let p = &mut ctx.accounts.profile;
        // 手動で Pubkey 比較
        if p.user_pubkey != ctx.accounts.user.key() {
            return Err(ProgramError::Custom(1).into());
        }
        // フィールド更新
        p.avatar_uri = uri.clone();
        p.avatar_updates = p.avatar_updates.saturating_add(1);

        // cache_acc は所有者チェックなくバイト操作
        let mut data = ctx.accounts.cache_acc.data.borrow_mut();
        data.fill(0); // キャッシュ丸ごとクリア
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Change<'info> {
    #[account(mut)]
    pub profile: Account<'info, ProfileData>,
    pub user: Signer<'info>,
    /// CHECK: キャッシュアカウント
    #[account(mut)]
    pub cache_acc: AccountInfo<'info>,
}

#[account]
pub struct ProfileData {
    pub user_pubkey: Pubkey,
    pub avatar_uri: String,
    pub avatar_updates: u64,
}
