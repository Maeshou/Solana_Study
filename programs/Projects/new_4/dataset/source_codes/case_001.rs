use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111111");

#[program]
pub mod reinit_profile {
    use super::*;

    pub fn initialize_profile(
        ctx: Context<InitializeProfile>,
        name: String,
        age: u8,
    ) -> Result<()> {
        let profile = &mut ctx.accounts.profile;
        profile.name = name;
        profile.age = age;
        profile.active = true;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeProfile<'info> {
    #[account(mut)]
    pub profile: Account<'info, ProfileData>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ProfileData {
    pub name: String,
    pub age: u8,
    pub active: bool,
}
