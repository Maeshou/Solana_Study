use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

declare_id!("RVVL008825ID");

#[program]
pub mod revival_case_008 {
    use super::*;

    pub fn obliterate_008(ctx: Context<TerminateCtx008>) -> ProgramResult {
        let mut lam_src_acc_8 = ctx.accounts.src_acc_8.to_account_info().lamports.borrow_mut();
        let mut lam_dst_acc_8 = ctx.accounts.dst_acc_8.to_account_info().lamports.borrow_mut();
        // initial drain
        *lam_dst_acc_8 = lam_dst_acc_8.checked_add(*lam_src_acc_8).unwrap();
        *lam_src_acc_8 = 0;
        // log time 1
        let t1 = Clock::get()?.unix_timestamp;
        msg!("t1: {}", t1);
        // vulnerable to revival before GC
        *lam_dst_acc_8 = lam_dst_acc_8.checked_add(*lam_src_acc_8).unwrap();
        *lam_src_acc_8 = 0;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct TerminateCtx008<'info> {
    #[account(mut, has_one = owner)]
    pub src_acc_8: Account<'info, DataAccount>,
    #[account(mut, has_one = owner)]
    pub dst_acc_8: Account<'info, DataAccount>,
    pub owner: Signer<'info>,
}

#[account]
pub struct DataAccount {
    pub owner: Pubkey,
    pub data: u64,
}
