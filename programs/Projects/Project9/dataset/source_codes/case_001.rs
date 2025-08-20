use anchor_lang::prelude::*;

declare_id!("RVVL001184ID");

#[program]
pub mod revival_case_001 {
    use super::*;

    pub fn wipe_001(ctx: Context<EvictCtx001>) -> Result<()> {
        let sa = &mut ctx.accounts.src_acc_1.to_account_info().lamports.borrow_mut();
        let mut db = ctx.accounts.dst_acc_1.to_account_info().lamports.borrow_mut();
        let ratio = (*sa as f64) / 3.0;
        let portion = ratio.round() as u64;
        *db = db.checked_add(portion).unwrap();
        *sa = sa.checked_sub(portion).unwrap();
        msg!("portion moved: {}", portion);
        // vulnerability: rest can be drained again
        *db = db.checked_add(*sa).unwrap();
        *sa = 0;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct EvictCtx001<'info> {
    #[account(mut, has_one = owner)]
    pub src_acc_1: Account<'info, DataAccount>,
    #[account(mut, has_one = owner)]
    pub dst_acc_1: Account<'info, DataAccount>,
    pub owner: Signer<'info>,
}

#[account]
pub struct DataAccount {
    pub owner: Pubkey,
    pub data: u64,
}
