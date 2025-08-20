use anchor_lang::prelude::*;

declare_id!("VulnProf9999999999999999999999999999999999");

#[program]
pub mod vuln_profile {
    pub fn set_bio(ctx: Context<SetBio>, bio: String) -> Result<()> {
        let p = &mut ctx.accounts.profile;
        // p.owner 未検証で誰でも更新可能
        p.bio = bio;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetBio<'info> {
    #[account(mut)]
    pub profile: Account<'info, Profile>,
}

#[account]
pub struct Profile {
    pub owner: Pubkey,
    pub bio: String,
}
