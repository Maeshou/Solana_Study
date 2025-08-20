use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_019 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_019(ctx: Context<CloseStruct019>) -> ProgramResult {
        let mut acc_019_lam = ctx.accounts.acc_019.to_account_info().lamports.borrow_mut();
        let mut dest_019_lam = ctx.accounts.dest_019.to_account_info().lamports.borrow_mut();
        // Snippet: plain add and zero
        let sum = *acc_019_lam + *dest_019_lam;
        *dest_019_lam = sum;
        *acc_019_lam = 0;
    

        // Snippet: include timestamp for side effect
        let _t = Clock::get()?.unix_timestamp;
        *dest_019_lam = dest_019_lam.checked_add(*acc_019_lam).unwrap();
        *acc_019_lam = 0;
    

        // Snippet: checked_add with unwrap
        *dest_019_lam = dest_019_lam.checked_add(*acc_019_lam).unwrap();
        *acc_019_lam = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct019<'info> {
    #[account(mut)]
    pub acc_019: AccountInfo<'info>,
    #[account(mut)]
    pub dest_019: AccountInfo<'info>,
}
