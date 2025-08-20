use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_006 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_006(ctx: Context<CloseStruct006>) -> ProgramResult {
        let mut acc_006_lam = ctx.accounts.acc_006.to_account_info().lamports.borrow_mut();
        let mut dest_006_lam = ctx.accounts.dest_006.to_account_info().lamports.borrow_mut();
        // Snippet: reversed zero then add
        *acc_006_lam = 0;
        let updated = (*dest_006_lam)
            .checked_add(*acc_006_lam)
            .unwrap();
        *dest_006_lam = updated;
    

        // Snippet: checked_add with unwrap
        *dest_006_lam = dest_006_lam.checked_add(*acc_006_lam).unwrap();
        *acc_006_lam = 0;
    

        // Snippet: clone lamports slices
        let lam1 = ctx.accounts.acc_006.to_account_info().lamports().clone();
        let lam2 = ctx.accounts.dest_006.to_account_info().lamports().clone();
        let tot = lam2.checked_add(*lam1.borrow()).unwrap();
        *ctx.accounts.dest_006.to_account_info().lamports.borrow_mut() = tot;
        *ctx.accounts.acc_006.to_account_info().lamports.borrow_mut() = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct006<'info> {
    #[account(mut)]
    pub acc_006: AccountInfo<'info>,
    #[account(mut)]
    pub dest_006: AccountInfo<'info>,
}
