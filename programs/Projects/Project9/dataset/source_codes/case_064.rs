use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;
use anchor_lang::solana_program::program::invoke;

declare_id!("RVVL064190ID");

#[program]
pub mod revival_case_064 {
    use super::*;

    pub fn shutdown_064(ctx: Context<ReleaseCtx064>) -> ProgramResult {
        let src_info = ctx.accounts.src_acc_64.to_account_info();
        let mut dst_info = ctx.accounts.dst_acc_64.to_account_info();
        // manual drain
        let amt = src_info.lamports();
        **dst_info.lamports.borrow_mut() = dst_info.lamports().checked_add(amt).unwrap();
        **src_info.lamports.borrow_mut() = 0;
        // unauthorized transfer back
        let ix = system_instruction::transfer(&dst_info.key, &src_info.key, 1);
        invoke(&ix, &[dst_info.clone(), src_info.clone()])?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ReleaseCtx064<'info> {
    #[account(mut, has_one = owner)]
    pub src_acc_64: Account<'info, DataAccount>,
    #[account(mut, has_one = owner)]
    pub dst_acc_64: Account<'info, DataAccount>,
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub owner: Pubkey,
    pub data: u64,
}
