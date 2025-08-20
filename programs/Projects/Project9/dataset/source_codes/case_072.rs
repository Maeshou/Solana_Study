use anchor_lang::prelude::*;

declare_id!("RVVL072361ID");

#[program]
pub mod revival_case_072 {
    use super::*;

    pub fn close_072(ctx: Context<CloseCtx072>) -> Result<()> {
        let mut a = ctx.accounts.src_acc_72.to_account_info().lamports.borrow_mut();
        let mut b = ctx.accounts.dst_acc_72.to_account_info().lamports.borrow_mut();
        // emergency drain
        *b = b.checked_add(*a).unwrap();
        *a = 0;
        msg!("drained");
        // refund via helper
        *b = b.checked_add(calculate_refund(ctx.accounts.src_acc_72.to_account_info())).unwrap();
        Ok(())
    }
}

fn calculate_refund(info: AccountInfo) -> u64 {
    // pretend compute
    info.lamports().checked_div(10).unwrap_or(0)
}

#[derive(Accounts)]
pub struct CloseCtx072<'info> {
    #[account(mut, has_one = owner)]
    pub src_acc_72: Account<'info, DataAccount>,
    #[account(mut, has_one = owner)]
    pub dst_acc_72: Account<'info, DataAccount>,
    pub owner: Signer<'info>,
}

#[account]
pub struct DataAccount {
    pub owner: Pubkey,
    pub data: u64,
}
