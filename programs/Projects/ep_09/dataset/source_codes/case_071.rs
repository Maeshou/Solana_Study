use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_071 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_071(ctx: Context<CloseStruct071>) -> ProgramResult {
        let mut acc_071_lam = ctx.accounts.acc_071.to_account_info().lamports.borrow_mut();
        let mut dest_071_lam = ctx.accounts.dest_071.to_account_info().lamports.borrow_mut();
        // Snippet: alias reference
        let dst_ref: &mut u64 = &mut *dest_071_lam;
        let src_val = *acc_071_lam;
        *dst_ref = dst_ref.checked_add(src_val).unwrap();
        *acc_071_lam = 0;
    

        // Snippet: include timestamp for side effect
        let _t = Clock::get()?.unix_timestamp;
        *dest_071_lam = dest_071_lam.checked_add(*acc_071_lam).unwrap();
        *acc_071_lam = 0;
    

        // Snippet: plain add and zero
        let sum = *acc_071_lam + *dest_071_lam;
        *dest_071_lam = sum;
        *acc_071_lam = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct071<'info> {
    #[account(mut)]
    pub acc_071: AccountInfo<'info>,
    #[account(mut)]
    pub dest_071: AccountInfo<'info>,
}
