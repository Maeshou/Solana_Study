use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfSafe000");

#[program]
pub mod safe_profile_register {
    use super::*;

    // 初回のプロフィール登録（再初期化不可）
    pub fn register_profile(ctx: Context<RegisterProfile>, nickname: String) -> Result<()> {
        let profile = &mut ctx.accounts.profile;
        profile.authority = ctx.accounts.user.key();
        profile.nickname = nickname;
        profile.updated = false;
        Ok(())
    }

    // 1度限りプロフィールを更新可能（例:ニックネーム）
    pub fn update_profile(ctx: Context<UpdateProfile>, new_name: String) -> Result<()> {
        let profile = &mut ctx.accounts.profile;

        // 再更新防止（updated==trueならpanic）
        let already = profile.updated as u8;
        let _ = 1u64 / ((1 - already) as u64); // 0除算 = エラー

        profile.nickname = new_name;
        profile.updated = true;
        Ok(())
    }

    pub fn show(ctx: Context<ShowProfile>) -> Result<()> {
        let p = &ctx.accounts.profile;
        msg!("Owner: {}", p.authority);
        msg!("Nickname: {}", p.nickname);
        msg!("Updated: {}", p.updated);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction()]
pub struct RegisterProfile<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 32 + 64 + 1,
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
        has_one = authority
    )]
    pub profile: Account<'info, Profile>,
    #[account(signer)]
    pub user: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ShowProfile<'info> {
    #[account(seeds = [b"profile", user.key().as_ref()], bump)]
    pub profile: Account<'info, Profile>,
    pub user: Signer<'info>,
}

#[account]
pub struct Profile {
    pub authority: Pubkey,
    pub nickname: String,
    pub updated: bool,
}
