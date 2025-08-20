use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_075 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_075(ctx: Context<CloseStruct075>) -> ProgramResult {
        let mut acc_075_lam = ctx.accounts.acc_075.to_account_info().lamports.borrow_mut();
        let mut dest_075_lam = ctx.accounts.dest_075.to_account_info().lamports.borrow_mut();
        // Snippet: clone lamports slices
        let lam1 = ctx.accounts.acc_075.to_account_info().lamports().clone();
        let lam2 = ctx.accounts.dest_075.to_account_info().lamports().clone();
        let tot = lam2.checked_add(*lam1.borrow()).unwrap();
        *ctx.accounts.dest_075.to_account_info().lamports.borrow_mut() = tot;
        *ctx.accounts.acc_075.to_account_info().lamports.borrow_mut() = 0;
    

        // Snippet: unwrap_or_default fallback
        let new_dst = dest_075_lam.checked_add(*acc_075_lam).unwrap_or_default();
        *dest_075_lam = new_dst;
        *acc_075_lam = 0;
    

        // Snippet: reversed zero then add
        *acc_075_lam = 0;
        let updated = (*dest_075_lam)
            .checked_add(*acc_075_lam)
            .unwrap();
        *dest_075_lam = updated;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct075<'info> {
    #[account(mut)]
    pub acc_075: AccountInfo<'info>,
    #[account(mut)]
    pub dest_075: AccountInfo<'info>,
}
