use anchor_lang::prelude::*;

declare_id!("VulnEx67000000000000000000000000000000000067");

#[program]
pub mod example67 {
    pub fn reward_milestone(ctx: Context<Ctx67>, milestone: u8) -> Result<()> {
        // bench_buf is unchecked
        ctx.accounts.bench_buf.data.borrow_mut()[0] = milestone;
        // user_account is has_one = user
        let ua = &mut ctx.accounts.user_account;
        ua.milestones.push(milestone);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx67<'info> {
    pub user: Signer<'info>,
    #[account(mut)]
    pub bench_buf: AccountInfo<'info>,
    #[account(mut, has_one = user)]
    pub user_account: Account<'info, UserAccount>,
}

#[account]
pub struct UserAccount {
    pub user: Pubkey,
    pub milestones: Vec<u8>,
}
