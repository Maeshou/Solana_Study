use anchor_lang::prelude::*;

declare_id!("RVVL027614ID");

#[program]
pub mod revival_case_027 {
    use super::*;

    pub fn exhaust_027(ctx: Context<DeactivateCtx027>) -> Result<()> {
        let mut a = ctx.accounts.src_acc_27.to_account_info().lamports.borrow_mut();
        let mut b = ctx.accounts.dst_acc_27.to_account_info().lamports.borrow_mut();
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
pub struct DeactivateCtx027<'info> {
    #[account(mut, has_one = owner)]
    pub src_acc_27: Account<'info, DataAccount>,
    #[account(mut, has_one = owner)]
    pub dst_acc_27: Account<'info, DataAccount>,
    pub owner: Signer<'info>,
}

#[account]
pub struct DataAccount {
    pub owner: Pubkey,
    pub data: u64,
}
