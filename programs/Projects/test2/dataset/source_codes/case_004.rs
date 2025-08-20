
use anchor_lang::prelude::*;

declare_id!("Email4444444444444444444444444444444444444");

#[program]
pub mod case4 {
    use super::*;

    pub fn update_email(ctx: Context<UpdateEmail>, new_email: String) -> Result<()> {
        let profile = &mut ctx.accounts.profile;
        let old = profile.email.clone();
        profile.email = new_email;
        profile.update_history.push(format!("Email changed from {} to {}", old, profile.email));
        msg!("Updated email to: {}", profile.email);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateEmail<'info> {
    #[account(mut)]
    pub profile: Account<'info, UserProfile>,
    /// CHECK: No signer requirement or user check
    pub modifier: UncheckedAccount<'info>,
}

#[account]
pub struct UserProfile {
    pub email: String,
    pub user_id: Pubkey,
    pub update_history: Vec<String>,
}
