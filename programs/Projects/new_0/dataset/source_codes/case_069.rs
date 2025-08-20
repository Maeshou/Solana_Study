use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfProfile1");

#[program]
pub mod user_profile_system {
    use super::*;

    // ユーザープロファイル初期化（PDAで一度きり）
    pub fn initialize_profile(
        ctx: Context<InitializeProfile>,
        name: String,
        bio: String,
    ) -> Result<()> {
        let acc = &mut ctx.accounts.profile;
        acc.owner = ctx.accounts.user.key();
        acc.name = name;
        acc.bio = bio;
        acc.experience = 0;
        acc.rank = 1;
        Ok(())
    }

    // 経験値加算
    pub fn gain_experience(ctx: Context<UpdateProfile>, points: u64) -> Result<()> {
        let acc = &mut ctx.accounts.profile;
        acc.experience = acc.experience.saturating_add(points);
        acc.rank = 1 + (acc.experience / 1000);
        Ok(())
    }

    // 参照のみ（メッセージ出力）
    pub fn view_profile(ctx: Context<UpdateProfile>) -> Result<()> {
        let acc = &ctx.accounts.profile;
        msg!("User: {}", acc.name);
        msg!("Bio : {}", acc.bio);
        msg!("XP  : {}", acc.experience);
        msg!("Rank: {}", acc.rank);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct InitializeProfile<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 32 + 64 + 256 + 8 + 8,
        seeds = [b"profile", user.key().as_ref()],
        bump
    )]
    pub profile: Account<'info, Profile>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateProfile<'info> {
    #[account(
        mut,
        seeds = [b"profile", user.key().as_ref()],
        bump,
        has_one = owner
    )]
    pub profile: Account<'info, Profile>,
    pub user: Signer<'info>,
}

#[account]
pub struct Profile {
    pub owner: Pubkey,
    pub name: String,       // 最大64文字
    pub bio: String,        // 最大256文字
    pub experience: u64,    // 経験値
    pub rank: u64,          // 1000XPごとに1ランクアップ
}
