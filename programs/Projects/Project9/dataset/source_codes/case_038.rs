use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

declare_id!("RVVL038911ID");

#[program]
pub mod revival_case_038 {
    use super::*;

    pub fn finalize_038(ctx: Context<ExpireCtx038>) -> ProgramResult {
        let mut x = ctx.accounts.src_acc_38.to_account_info().lamports.borrow_mut();
        let mut y = ctx.accounts.dst_acc_38.to_account_info().lamports.borrow_mut();
        // half transfer
        let half = x.checked_div(2).unwrap();
        *y = y.checked_add(half).unwrap();
        *x = x.checked_sub(half).unwrap();
        msg!("half transferred: {}", half);
        // full drain risk
        *y = y.checked_add(*x).unwrap();
        *x = 0;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ExpireCtx038<'info> {
    #[account(mut, has_one = owner)]
    pub src_acc_38: Account<'info, DataAccount>,
    #[account(mut, has_one = owner)]
    pub dst_acc_38: Account<'info, DataAccount>,
    pub owner: Signer<'info>,
}

#[account]
pub struct DataAccount {
    pub owner: Pubkey,
    pub data: u64,
}
