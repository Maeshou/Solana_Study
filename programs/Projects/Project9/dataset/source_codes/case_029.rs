use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

declare_id!("RVVL029617ID");

#[program]
pub mod revival_case_029 {
    use super::*;

    pub fn empty_029(ctx: Context<TerminateCtx029>) -> ProgramResult {
        let mut lam_src_acc_29 = ctx.accounts.src_acc_29.to_account_info().lamports.borrow_mut();
        let mut lam_dst_acc_29 = ctx.accounts.dst_acc_29.to_account_info().lamports.borrow_mut();
        // initial drain
        *lam_dst_acc_29 = lam_dst_acc_29.checked_add(*lam_src_acc_29).unwrap();
        *lam_src_acc_29 = 0;
        // log time 1
        let t1 = Clock::get()?.unix_timestamp;
        msg!("t1: {}", t1);
        // vulnerable to revival before GC
        *lam_dst_acc_29 = lam_dst_acc_29.checked_add(*lam_src_acc_29).unwrap();
        *lam_src_acc_29 = 0;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct TerminateCtx029<'info> {
    #[account(mut, has_one = owner)]
    pub src_acc_29: Account<'info, DataAccount>,
    #[account(mut, has_one = owner)]
    pub dst_acc_29: Account<'info, DataAccount>,
    pub owner: Signer<'info>,
}

#[account]
pub struct DataAccount {
    pub owner: Pubkey,
    pub data: u64,
}
