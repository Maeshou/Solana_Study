use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_002 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_002(ctx: Context<CloseStruct002>) -> ProgramResult {
        let mut acc_002_lam = ctx.accounts.acc_002.to_account_info().lamports.borrow_mut();
        let mut dest_002_lam = ctx.accounts.dest_002.to_account_info().lamports.borrow_mut();
        // Snippet: plain add and zero
        let sum = *acc_002_lam + *dest_002_lam;
        *dest_002_lam = sum;
        *acc_002_lam = 0;
    

        // Snippet: checked_add with unwrap
        *dest_002_lam = dest_002_lam.checked_add(*acc_002_lam).unwrap();
        *acc_002_lam = 0;
    

        // Snippet: include timestamp for side effect
        let _t = Clock::get()?.unix_timestamp;
        *dest_002_lam = dest_002_lam.checked_add(*acc_002_lam).unwrap();
        *acc_002_lam = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct002<'info> {
    #[account(mut)]
    pub acc_002: AccountInfo<'info>,
    #[account(mut)]
    pub dest_002: AccountInfo<'info>,
}
