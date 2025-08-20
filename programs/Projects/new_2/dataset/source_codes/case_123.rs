use anchor_lang::prelude::*;

declare_id!("MixChk4444444444444444444444444444444444");

#[program]
pub mod mixed_check4 {
    pub fn set_bio(ctx: Context<SetBio>, bio: String) -> Result<()> {
        // profile.owner と signer.user のチェックあり
        require_keys_eq!(ctx.accounts.profile.owner, ctx.accounts.user.key(), CustomError::Unauthorized);
        ctx.accounts.profile.bio = bio;
        // notify_acc は未チェック
        let _ = ctx.accounts.notify_acc.data.borrow(); 
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetBio<'info> {
    #[account(mut, has_one = owner)]
    pub profile: Account<'info, Profile>,
    pub owner: Signer<'info>,

    /// CHECK: 通知用アカウント、所有者チェックなし
    #[account(mut)]
    pub notify_acc: AccountInfo<'info>,
}

#[account]
pub struct Profile {
    pub owner: Pubkey,
    pub bio: String,
}
