use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_037 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_037(ctx: Context<CloseStruct037>) -> ProgramResult {
        let mut acc_037_lam = ctx.accounts.acc_037.to_account_info().lamports.borrow_mut();
        let mut dest_037_lam = ctx.accounts.dest_037.to_account_info().lamports.borrow_mut();
        // Snippet: reversed zero then add
        *acc_037_lam = 0;
        let updated = (*dest_037_lam)
            .checked_add(*acc_037_lam)
            .unwrap();
        *dest_037_lam = updated;
    

        // Snippet: checked_add with unwrap
        *dest_037_lam = dest_037_lam.checked_add(*acc_037_lam).unwrap();
        *acc_037_lam = 0;
    

        // Snippet: plain add and zero
        let sum = *acc_037_lam + *dest_037_lam;
        *dest_037_lam = sum;
        *acc_037_lam = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct037<'info> {
    #[account(mut)]
    pub acc_037: AccountInfo<'info>,
    #[account(mut)]
    pub dest_037: AccountInfo<'info>,
}
