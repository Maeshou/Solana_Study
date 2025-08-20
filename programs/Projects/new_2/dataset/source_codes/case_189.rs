use anchor_lang::prelude::*;

declare_id!("OwnChkD000000000000000000000000000000001");

#[program]
pub mod avatar_update {
    pub fn set_avatar(
        ctx: Context<SetAvatar>,
        uri: String,
    ) -> Result<()> {
        let prof = &mut ctx.accounts.profile;
        // 属性レベルで owner を検証
        prof.avatar_uri = uri.clone();
        prof.updated_at = Clock::get()?.unix_timestamp;

        // cache_acc は unchecked で丸ごとクリア
        ctx.accounts.cache_acc.data.borrow_mut().fill(0);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetAvatar<'info> {
    #[account(mut, has_one = owner)]
    pub profile: Account<'info, Profile>,
    pub owner: Signer<'info>,
    /// CHECK: キャッシュアカウント、所有者検証なし
    #[account(mut)]
    pub cache_acc: AccountInfo<'info>,
    pub clock: Sysvar<'info, Clock>,
}

#[account]
pub struct Profile {
    pub owner: Pubkey,
    pub avatar_uri: String,
    pub updated_at: i64,
}
