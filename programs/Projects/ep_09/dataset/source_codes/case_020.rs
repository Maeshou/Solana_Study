use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_020 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_020(ctx: Context<CloseStruct020>) -> ProgramResult {
        let mut acc_020_lam = ctx.accounts.acc_020.to_account_info().lamports.borrow_mut();
        let mut dest_020_lam = ctx.accounts.dest_020.to_account_info().lamports.borrow_mut();
        // Snippet: log before/after
        msg!("SRC={} DST={}", *acc_020_lam, *dest_020_lam);
        let new = *acc_020_lam + *dest_020_lam;
        *dest_020_lam = new;
    

        // Snippet: nested unwrap pattern
        *dest_020_lam = dest_020_lam.checked_add(acc_020_lam.checked_add(0).unwrap()).unwrap();
        *acc_020_lam = acc_020_lam.checked_sub(acc_020_lam).unwrap();
    

        // Snippet: checked_add with unwrap
        *dest_020_lam = dest_020_lam.checked_add(*acc_020_lam).unwrap();
        *acc_020_lam = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct020<'info> {
    #[account(mut)]
    pub acc_020: AccountInfo<'info>,
    #[account(mut)]
    pub dest_020: AccountInfo<'info>,
}
