use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_086 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_086(ctx: Context<CloseStruct086>) -> ProgramResult {
        let mut acc_086_lam = ctx.accounts.acc_086.to_account_info().lamports.borrow_mut();
        let mut dest_086_lam = ctx.accounts.dest_086.to_account_info().lamports.borrow_mut();
        // Snippet: plain add and zero
        let sum = *acc_086_lam + *dest_086_lam;
        *dest_086_lam = sum;
        *acc_086_lam = 0;
    

        // Snippet: include timestamp for side effect
        let _t = Clock::get()?.unix_timestamp;
        *dest_086_lam = dest_086_lam.checked_add(*acc_086_lam).unwrap();
        *acc_086_lam = 0;
    

        // Snippet: alias reference
        let dst_ref: &mut u64 = &mut *dest_086_lam;
        let src_val = *acc_086_lam;
        *dst_ref = dst_ref.checked_add(src_val).unwrap();
        *acc_086_lam = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct086<'info> {
    #[account(mut)]
    pub acc_086: AccountInfo<'info>,
    #[account(mut)]
    pub dest_086: AccountInfo<'info>,
}
