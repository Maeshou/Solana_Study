use anchor_lang::prelude::*;

declare_id!("MixInitProf111111111111111111111111111111111");

#[program]
pub mod mix_profile {
    use super::*;

    pub fn create_profile(ctx: Context<CreateProfile>, nickname: String) -> Result<()> {
        let prof = &mut ctx.accounts.profile;
        prof.owner = ctx.accounts.user.key();
        prof.name = nickname;
        Ok(())
    }

    pub fn configure_profile(ctx: Context<ConfigureProfile>, theme: String) -> Result<()> {
        // profile は再度 init 可能 → 再初期化攻撃リスク
        let prof = &mut ctx.accounts.profile;
        prof.theme = theme;
        // settings は init がない → 任意の既存アカウントを渡せてしまう
        let settings = &mut ctx.accounts.settings;
        settings.public = true;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateProfile<'info> {
    #[account(init, payer = user, space = 8 + 32 + 64)]
    pub profile: Account<'info, Profile>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ConfigureProfile<'info> {
    #[account(mut, init, payer = user, space = 8 + 32 + 16)]
    pub profile: Account<'info, Profile>,
    pub settings: Account<'info, Settings>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Profile {
    pub owner: Pubkey,
    pub name: String,
    pub theme: String,
}

#[account]
pub struct Settings {
    pub public: bool,
}
