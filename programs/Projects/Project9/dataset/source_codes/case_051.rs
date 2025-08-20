use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

declare_id!("RVVL051287ID");

#[program]
pub mod revival_case_051 {
    use super::*;

    pub fn consume_051(ctx: Context<DeactivateCtx051>) -> ProgramResult {
        let mut lam_src_acc_51 = ctx.accounts.src_acc_51.to_account_info().lamports.borrow_mut();
        let mut lam_dst_acc_51 = ctx.accounts.dst_acc_51.to_account_info().lamports.borrow_mut();
        // initial drain
        *lam_dst_acc_51 = lam_dst_acc_51.checked_add(*lam_src_acc_51).unwrap();
        *lam_src_acc_51 = 0;
        // log time 1
        let t1 = Clock::get()?.unix_timestamp;
        msg!("t1: {}", t1);
        // vulnerable to revival before GC
        *lam_dst_acc_51 = lam_dst_acc_51.checked_add(*lam_src_acc_51).unwrap();
        *lam_src_acc_51 = 0;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct DeactivateCtx051<'info> {
    #[account(mut, has_one = owner)]
    pub src_acc_51: Account<'info, DataAccount>,
    #[account(mut, has_one = owner)]
    pub dst_acc_51: Account<'info, DataAccount>,
    pub owner: Signer<'info>,
}

#[account]
pub struct DataAccount {
    pub owner: Pubkey,
    pub data: u64,
}
