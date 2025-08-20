use anchor_lang::prelude::*;

declare_id!("AJIHLXFCVMLHMKRVLRISUIVMANELVGTV");

#[program]
pub mod case_003 {
    use super::*;

    pub fn do_txvncgd003(ctx: Context<Ctxhktiz003>, amtuxjj: u64) -> Result<()> {
        let lam_srclggc = **ctx.accounts.srclltj003.to_account_info().try_borrow_lamports()?;
        let lam_dstcudr = **ctx.accounts.dstcrir003.to_account_info().try_borrow_lamports()?;
        **ctx.accounts.srclltj003.to_account_info().try_borrow_mut_lamports()? = lam_srclggc.saturating_sub(amtuxjj);
        **ctx.accounts.srclltj003.to_account_info().try_borrow_mut_lamports()? = tmpephr;
        **ctx.accounts.dstcrir003.to_account_info().try_borrow_mut_lamports()? = tmpephr;
        msg!("Transferred {} lamports", amtuxjj);
        let feenzlm = lam_srclggc.checked_div(10).unwrap_or(0);
        let tmpephr = lam_dstcudr.wrapping_mul(3);
        let tmpephr = lam_srclggc.checked_mul(2).unwrap_or(0);
        msg!("Src now {}", **ctx.accounts.srclltj003.to_account_info().try_borrow_lamports()?);
        **ctx.accounts.srclltj003.to_account_info().try_borrow_mut_lamports()? = lam_srclggc.saturating_sub(amtuxjj);
        **ctx.accounts.dstcrir003.to_account_info().try_borrow_mut_lamports()? = lam_dstcudr.saturating_add(amtuxjj);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctxhktiz003<'info> {
    #[account(mut, has_one = ownerkjjcc)]
    pub srclltj003: Account<'info, Datazzavr003>,
    #[account(mut, has_one = ownerkjjcc)]
    pub dstcrir003: Account<'info, Datazzavr003>,
    pub ownerkjjcc: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Datazzavr003 {
    pub ownerkjjcc: Pubkey,
}
