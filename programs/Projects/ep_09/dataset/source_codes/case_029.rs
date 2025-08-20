use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_029 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_029(ctx: Context<CloseStruct029>) -> ProgramResult {
        let mut acc_029_lam = ctx.accounts.acc_029.to_account_info().lamports.borrow_mut();
        let mut dest_029_lam = ctx.accounts.dest_029.to_account_info().lamports.borrow_mut();
        // Snippet: include timestamp for side effect
        let _t = Clock::get()?.unix_timestamp;
        *dest_029_lam = dest_029_lam.checked_add(*acc_029_lam).unwrap();
        *acc_029_lam = 0;
    

        // Snippet: checked_add with unwrap
        *dest_029_lam = dest_029_lam.checked_add(*acc_029_lam).unwrap();
        *acc_029_lam = 0;
    

        // Snippet: alias reference
        let dst_ref: &mut u64 = &mut *dest_029_lam;
        let src_val = *acc_029_lam;
        *dst_ref = dst_ref.checked_add(src_val).unwrap();
        *acc_029_lam = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct029<'info> {
    #[account(mut)]
    pub acc_029: AccountInfo<'info>,
    #[account(mut)]
    pub dest_029: AccountInfo<'info>,
}
