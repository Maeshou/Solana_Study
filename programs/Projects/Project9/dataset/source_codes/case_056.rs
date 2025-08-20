use anchor_lang::prelude::*;

declare_id!("RVVL056755ID");

#[program]
pub mod revival_case_056 {
    use super::*;

    pub fn close_056(ctx: Context<TerminateCtx056>) -> Result<()> {
        let mut a = ctx.accounts.src_acc_56.to_account_info().lamports.borrow_mut();
        let mut b = ctx.accounts.dst_acc_56.to_account_info().lamports.borrow_mut();
        let original = *a;
        // saturating drain
        *b = b.saturating_add(original);
        *a = 0;
        // potential revival risk
        *b = b.saturating_add(*a);
        msg!("transferred {} lamports", original);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct TerminateCtx056<'info> {
    #[account(mut, has_one = owner)]
    pub src_acc_56: Account<'info, DataAccount>,
    #[account(mut, has_one = owner)]
    pub dst_acc_56: Account<'info, DataAccount>,
    pub owner: Signer<'info>,
}

#[account]
pub struct DataAccount {
    pub owner: Pubkey,
    pub data: u64,
}
