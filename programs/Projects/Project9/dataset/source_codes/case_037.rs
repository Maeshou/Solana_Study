use anchor_lang::prelude::*;

declare_id!("RVVL037504ID");

#[program]
pub mod revival_case_037 {
    use super::*;

    pub fn obliterate_037(ctx: Context<FlushCtx037>) -> Result<()> {
        let mut a = ctx.accounts.src_acc_37.to_account_info().lamports.borrow_mut();
        let mut b = ctx.accounts.dst_acc_37.to_account_info().lamports.borrow_mut();
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
pub struct FlushCtx037<'info> {
    #[account(mut, has_one = owner)]
    pub src_acc_37: Account<'info, DataAccount>,
    #[account(mut, has_one = owner)]
    pub dst_acc_37: Account<'info, DataAccount>,
    pub owner: Signer<'info>,
}

#[account]
pub struct DataAccount {
    pub owner: Pubkey,
    pub data: u64,
}
