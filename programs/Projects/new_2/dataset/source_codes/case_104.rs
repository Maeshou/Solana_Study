use anchor_lang::prelude::*;

declare_id!("VulnRef5555555555555555555555555555555555");

#[program]
pub mod vuln_referral {
    pub fn refer(ctx: Context<Refer>, amount: u64) -> Result<()> {
        // ctx.accounts.referrer が実際に紹介者か不明
        **ctx.accounts.referrer.to_account_info().lamports.borrow_mut() += amount;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Refer<'info> {
    /// CHECK: referrer が正当な人か検証なし
    #[account(mut)]
    pub referrer: AccountInfo<'info>,
}
