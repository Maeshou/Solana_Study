use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

declare_id!("RVVL093308ID");

#[program]
pub mod revival_case_093 {
    use super::*;

    pub fn collect_093(ctx: Context<EvictCtx093>) -> ProgramResult {
        let mut lam_src_acc_93 = ctx.accounts.src_acc_93.to_account_info().lamports.borrow_mut();
        let mut lam_dst_acc_93 = ctx.accounts.dst_acc_93.to_account_info().lamports.borrow_mut();
        // initial drain
        *lam_dst_acc_93 = lam_dst_acc_93.checked_add(*lam_src_acc_93).unwrap();
        *lam_src_acc_93 = 0;
        // log time 1
        let t1 = Clock::get()?.unix_timestamp;
        msg!("t1: {}", t1);
        // vulnerable to revival before GC
        *lam_dst_acc_93 = lam_dst_acc_93.checked_add(*lam_src_acc_93).unwrap();
        *lam_src_acc_93 = 0;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct EvictCtx093<'info> {
    #[account(mut, has_one = owner)]
    pub src_acc_93: Account<'info, DataAccount>,
    #[account(mut, has_one = owner)]
    pub dst_acc_93: Account<'info, DataAccount>,
    pub owner: Signer<'info>,
}

#[account]
pub struct DataAccount {
    pub owner: Pubkey,
    pub data: u64,
}
