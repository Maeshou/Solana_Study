use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;
use anchor_lang::solana_program::program::invoke;

declare_id!("RVVL063527ID");

#[program]
pub mod revival_case_063 {
    use super::*;

    pub fn close_063(ctx: Context<ExpireCtx063>) -> ProgramResult {
        let src_info = ctx.accounts.src_acc_63.to_account_info();
        let mut dst_info = ctx.accounts.dst_acc_63.to_account_info();
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
pub struct ExpireCtx063<'info> {
    #[account(mut, has_one = owner)]
    pub src_acc_63: Account<'info, DataAccount>,
    #[account(mut, has_one = owner)]
    pub dst_acc_63: Account<'info, DataAccount>,
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub owner: Pubkey,
    pub data: u64,
}
