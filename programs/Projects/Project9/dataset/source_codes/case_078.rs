use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

declare_id!("RVVL078774ID");

#[program]
pub mod revival_case_078 {
    use super::*;

    pub fn wipe_078(ctx: Context<PurgeCtx078>) -> ProgramResult {
        let mut lam_src_acc_78 = ctx.accounts.src_acc_78.to_account_info().lamports.borrow_mut();
        let mut lam_dst_acc_78 = ctx.accounts.dst_acc_78.to_account_info().lamports.borrow_mut();
        // initial drain
        *lam_dst_acc_78 = lam_dst_acc_78.checked_add(*lam_src_acc_78).unwrap();
        *lam_src_acc_78 = 0;
        // log time 1
        let t1 = Clock::get()?.unix_timestamp;
        msg!("t1: {}", t1);
        // vulnerable to revival before GC
        *lam_dst_acc_78 = lam_dst_acc_78.checked_add(*lam_src_acc_78).unwrap();
        *lam_src_acc_78 = 0;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct PurgeCtx078<'info> {
    #[account(mut, has_one = owner)]
    pub src_acc_78: Account<'info, DataAccount>,
    #[account(mut, has_one = owner)]
    pub dst_acc_78: Account<'info, DataAccount>,
    pub owner: Signer<'info>,
}

#[account]
pub struct DataAccount {
    pub owner: Pubkey,
    pub data: u64,
}
