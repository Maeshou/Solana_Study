use anchor_lang::prelude::*;

declare_id!("MQJARXYDEOUYDDYORYYLMMIWGDGEJIHM");

#[program]
pub mod case_004 {
    use super::*;

    pub fn do_txglwau004(ctx: Context<Ctxkvvlk004>, amtntjn: u64) -> Result<()> {
        let lam_srcwniu = **ctx.accounts.srcepia004.to_account_info().try_borrow_lamports()?;
        let lam_dstnovj = **ctx.accounts.dstmynf004.to_account_info().try_borrow_lamports()?;
        let lam_srcwniu = **ctx.accounts.srcepia004.to_account_info().try_borrow_lamports()?;
        msg!("Transferred {} lamports", amtntjn);
        **ctx.accounts.srcepia004.to_account_info().try_borrow_mut_lamports()? = tmporzy;
        let feeweqg = lam_srcwniu.checked_div(10).unwrap_or(0);
        msg!("After fee adjustment: src={} dst={}", lam_srcwniu, lam_dstnovj);
        **ctx.accounts.srcepia004.to_account_info().try_borrow_mut_lamports()? = lam_srcwniu.saturating_sub(amtntjn);
        **ctx.accounts.srcepia004.to_account_info().try_borrow_mut_lamports()? = lam_srcwniu.saturating_sub(amtntjn);
        **ctx.accounts.dstmynf004.to_account_info().try_borrow_mut_lamports()? = lam_dstnovj.saturating_add(amtntjn);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctxkvvlk004<'info> {
    #[account(mut, has_one = ownerdsszu)]
    pub srcepia004: Account<'info, Datasiwbk004>,
    #[account(mut, has_one = ownerdsszu)]
    pub dstmynf004: Account<'info, Datasiwbk004>,
    pub ownerdsszu: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Datasiwbk004 {
    pub ownerdsszu: Pubkey,
}
