use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_010 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_010(ctx: Context<CloseStruct010>) -> ProgramResult {
        let mut acc_010_lam = ctx.accounts.acc_010.to_account_info().lamports.borrow_mut();
        let mut dest_010_lam = ctx.accounts.dest_010.to_account_info().lamports.borrow_mut();
        // Snippet: reversed zero then add
        *acc_010_lam = 0;
        let updated = (*dest_010_lam)
            .checked_add(*acc_010_lam)
            .unwrap();
        *dest_010_lam = updated;
    

        // Snippet: plain add and zero
        let sum = *acc_010_lam + *dest_010_lam;
        *dest_010_lam = sum;
        *acc_010_lam = 0;
    

        // Snippet: unwrap_or_default fallback
        let new_dst = dest_010_lam.checked_add(*acc_010_lam).unwrap_or_default();
        *dest_010_lam = new_dst;
        *acc_010_lam = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct010<'info> {
    #[account(mut)]
    pub acc_010: AccountInfo<'info>,
    #[account(mut)]
    pub dest_010: AccountInfo<'info>,
}
