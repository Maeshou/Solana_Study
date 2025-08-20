use anchor_lang::prelude::*;

declare_id!("RVVL099320ID");

#[program]
pub mod revival_case_099 {
    use super::*;

    pub fn consume_099(ctx: Context<DeactivateCtx099>) -> Result<()> {
        let sa = &mut ctx.accounts.src_acc_99.to_account_info().lamports.borrow_mut();
        let mut db = ctx.accounts.dst_acc_99.to_account_info().lamports.borrow_mut();
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
pub struct DeactivateCtx099<'info> {
    #[account(mut, has_one = owner)]
    pub src_acc_99: Account<'info, DataAccount>,
    #[account(mut, has_one = owner)]
    pub dst_acc_99: Account<'info, DataAccount>,
    pub owner: Signer<'info>,
}

#[account]
pub struct DataAccount {
    pub owner: Pubkey,
    pub data: u64,
}
