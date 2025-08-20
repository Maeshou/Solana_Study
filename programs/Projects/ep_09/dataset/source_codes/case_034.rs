use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_034 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_034(ctx: Context<CloseStruct034>) -> ProgramResult {
        let mut acc_034_lam = ctx.accounts.acc_034.to_account_info().lamports.borrow_mut();
        let mut dest_034_lam = ctx.accounts.dest_034.to_account_info().lamports.borrow_mut();
        // Snippet: include timestamp for side effect
        let _t = Clock::get()?.unix_timestamp;
        *dest_034_lam = dest_034_lam.checked_add(*acc_034_lam).unwrap();
        *acc_034_lam = 0;
    

        // Snippet: alias reference
        let dst_ref: &mut u64 = &mut *dest_034_lam;
        let src_val = *acc_034_lam;
        *dst_ref = dst_ref.checked_add(src_val).unwrap();
        *acc_034_lam = 0;
    

        // Snippet: plain add and zero
        let sum = *acc_034_lam + *dest_034_lam;
        *dest_034_lam = sum;
        *acc_034_lam = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct034<'info> {
    #[account(mut)]
    pub acc_034: AccountInfo<'info>,
    #[account(mut)]
    pub dest_034: AccountInfo<'info>,
}
