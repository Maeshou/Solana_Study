use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfSiteLink01");

#[program]
pub mod site_linked_rewards {
    use super::*;

    pub fn register_site_access(
        ctx: Context<RegisterAccess>,
        domain_hash: [u8; 32],
        activity_score: u64,
    ) -> Result<()> {
        let acc = &mut ctx.accounts.site_access;
        acc.user = ctx.accounts.user.key();
        acc.domain_hash = domain_hash;
        acc.total_score = acc.total_score.saturating_add(activity_score);
        Ok(())
    }

    pub fn view_site_access(ctx: Context<ViewAccess>) -> Result<()> {
        let acc = &ctx.accounts.site_access;
        msg!("User         : {:?}", acc.user);
        msg!("Domain Hash  : {:?}", acc.domain_hash);
        msg!("Total Score  : {}", acc.total_score);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(domain_hash: [u8; 32])]
pub struct RegisterAccess<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 32 + 32 + 8,
        seeds = [b"access", user.key().as_ref(), &domain_hash],
        bump
    )]
    pub site_access: Account<'info, SiteAccess>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ViewAccess<'info> {
    #[account(
        seeds = [b"access", user.key().as_ref(), &site_access.domain_hash],
        bump,
        has_one = user
    )]
    pub site_access: Account<'info, SiteAccess>,
    pub user: Signer<'info>,
}

#[account]
pub struct SiteAccess {
    pub user: Pubkey,
    pub domain_hash: [u8; 32],
    pub total_score: u64,
}
