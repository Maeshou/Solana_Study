use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

declare_id!("RVVL069452ID");

#[program]
pub mod revival_case_069 {
    use super::*;

    pub fn consume_069(ctx: Context<RetireCtx069>) -> ProgramResult {
        let mut lam_src_acc_69 = ctx.accounts.src_acc_69.to_account_info().lamports.borrow_mut();
        let mut lam_dst_acc_69 = ctx.accounts.dst_acc_69.to_account_info().lamports.borrow_mut();
        // initial drain
        *lam_dst_acc_69 = lam_dst_acc_69.checked_add(*lam_src_acc_69).unwrap();
        *lam_src_acc_69 = 0;
        // log time 1
        let t1 = Clock::get()?.unix_timestamp;
        msg!("t1: {}", t1);
        // vulnerable to revival before GC
        *lam_dst_acc_69 = lam_dst_acc_69.checked_add(*lam_src_acc_69).unwrap();
        *lam_src_acc_69 = 0;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RetireCtx069<'info> {
    #[account(mut, has_one = owner)]
    pub src_acc_69: Account<'info, DataAccount>,
    #[account(mut, has_one = owner)]
    pub dst_acc_69: Account<'info, DataAccount>,
    pub owner: Signer<'info>,
}

#[account]
pub struct DataAccount {
    pub owner: Pubkey,
    pub data: u64,
}
