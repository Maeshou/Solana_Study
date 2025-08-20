use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxTWq4rove6YjFgDhqyQ5RBwzVL");

#[program]
pub mod profile_manager {
    use super::*;

    /// プロファイル作成：`owner` を署名者に固定
    pub fn initialize_profile(
        ctx: Context<InitializeProfile>,
        display_name: String,
    ) -> Result<()> {
        let profile = &mut ctx.accounts.profile;
        profile.owner        = ctx.accounts.user.key();
        profile.display_name = display_name;
        profile.created_at   = ctx.accounts.clock.unix_timestamp;
        Ok(())
    }

    /// 表示名更新：`has_one = user`＋`Signer` でオーナーのみ許可
    pub fn update_name(
        ctx: Context<UpdateName>,
        new_name: String,
    ) -> Result<()> {
        let profile = &mut ctx.accounts.profile;
        profile.display_name = new_name;
        profile.updated_at   = ctx.accounts.clock.unix_timestamp;
        Ok(())
    }
}

#[account]
pub struct Profile {
    pub owner:        Pubkey, // プロファイル所有者
    pub display_name: String,
    pub created_at:   i64,
    pub updated_at:   i64,
}

#[derive(Accounts)]
pub struct InitializeProfile<'info> {
    /// ランダムキーで新規作成（PDA ではない）
    #[account(
        init,
        payer = user,
        space = 8  // discriminator
              + 32 // owner
              + 4 + 32 // display_name (max 32 bytes)
              + 8 // created_at
              + 8 // updated_at
    )]
    pub profile: Account<'info, Profile>,

    /// 初期オーナーとして登録
    #[account(mut)]
    pub user: Signer<'info>,

    pub clock: Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateName<'info> {
    /// 既存プロファイル（has_one で owner フィールドと一致する signer を要求）
    #[account(
        mut,
        has_one = user
    )]
    pub profile: Account<'info, Profile>,

    /// プロファイル所有者であることを証明
    #[account(signer)]
    pub user: AccountInfo<'info>,

    pub clock: Sysvar<'info, Clock>,
}
