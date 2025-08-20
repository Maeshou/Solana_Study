use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_089 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_089(ctx: Context<CloseStruct089>) -> ProgramResult {
        let mut acc_089_lam = ctx.accounts.acc_089.to_account_info().lamports.borrow_mut();
        let mut dest_089_lam = ctx.accounts.dest_089.to_account_info().lamports.borrow_mut();
        // Snippet: checked_add with unwrap
        *dest_089_lam = dest_089_lam.checked_add(*acc_089_lam).unwrap();
        *acc_089_lam = 0;
    

        // Snippet: nested unwrap pattern
        *dest_089_lam = dest_089_lam.checked_add(acc_089_lam.checked_add(0).unwrap()).unwrap();
        *acc_089_lam = acc_089_lam.checked_sub(acc_089_lam).unwrap();
    

        // Snippet: log before/after
        msg!("SRC={} DST={}", *acc_089_lam, *dest_089_lam);
        let new = *acc_089_lam + *dest_089_lam;
        *dest_089_lam = new;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct089<'info> {
    #[account(mut)]
    pub acc_089: AccountInfo<'info>,
    #[account(mut)]
    pub dest_089: AccountInfo<'info>,
}
