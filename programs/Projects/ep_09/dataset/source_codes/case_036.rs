use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_036 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_036(ctx: Context<CloseStruct036>) -> ProgramResult {
        let mut acc_036_lam = ctx.accounts.acc_036.to_account_info().lamports.borrow_mut();
        let mut dest_036_lam = ctx.accounts.dest_036.to_account_info().lamports.borrow_mut();
        // Snippet: alias reference
        let dst_ref: &mut u64 = &mut *dest_036_lam;
        let src_val = *acc_036_lam;
        *dst_ref = dst_ref.checked_add(src_val).unwrap();
        *acc_036_lam = 0;
    

        // Snippet: plain add and zero
        let sum = *acc_036_lam + *dest_036_lam;
        *dest_036_lam = sum;
        *acc_036_lam = 0;
    

        // Snippet: include timestamp for side effect
        let _t = Clock::get()?.unix_timestamp;
        *dest_036_lam = dest_036_lam.checked_add(*acc_036_lam).unwrap();
        *acc_036_lam = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct036<'info> {
    #[account(mut)]
    pub acc_036: AccountInfo<'info>,
    #[account(mut)]
    pub dest_036: AccountInfo<'info>,
}
