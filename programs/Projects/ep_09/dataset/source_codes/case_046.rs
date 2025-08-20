use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_046 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_046(ctx: Context<CloseStruct046>) -> ProgramResult {
        let mut acc_046_lam = ctx.accounts.acc_046.to_account_info().lamports.borrow_mut();
        let mut dest_046_lam = ctx.accounts.dest_046.to_account_info().lamports.borrow_mut();
        // Snippet: checked_add with unwrap
        *dest_046_lam = dest_046_lam.checked_add(*acc_046_lam).unwrap();
        *acc_046_lam = 0;
    

        // Snippet: unwrap_or_default fallback
        let new_dst = dest_046_lam.checked_add(*acc_046_lam).unwrap_or_default();
        *dest_046_lam = new_dst;
        *acc_046_lam = 0;
    

        // Snippet: plain add and zero
        let sum = *acc_046_lam + *dest_046_lam;
        *dest_046_lam = sum;
        *acc_046_lam = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct046<'info> {
    #[account(mut)]
    pub acc_046: AccountInfo<'info>,
    #[account(mut)]
    pub dest_046: AccountInfo<'info>,
}
