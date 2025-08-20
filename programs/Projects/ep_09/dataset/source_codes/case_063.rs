use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_063 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_063(ctx: Context<CloseStruct063>) -> ProgramResult {
        let mut acc_063_lam = ctx.accounts.acc_063.to_account_info().lamports.borrow_mut();
        let mut dest_063_lam = ctx.accounts.dest_063.to_account_info().lamports.borrow_mut();
        // Snippet: clone lamports slices
        let lam1 = ctx.accounts.acc_063.to_account_info().lamports().clone();
        let lam2 = ctx.accounts.dest_063.to_account_info().lamports().clone();
        let tot = lam2.checked_add(*lam1.borrow()).unwrap();
        *ctx.accounts.dest_063.to_account_info().lamports.borrow_mut() = tot;
        *ctx.accounts.acc_063.to_account_info().lamports.borrow_mut() = 0;
    

        // Snippet: reversed zero then add
        *acc_063_lam = 0;
        let updated = (*dest_063_lam)
            .checked_add(*acc_063_lam)
            .unwrap();
        *dest_063_lam = updated;
    

        // Snippet: nested unwrap pattern
        *dest_063_lam = dest_063_lam.checked_add(acc_063_lam.checked_add(0).unwrap()).unwrap();
        *acc_063_lam = acc_063_lam.checked_sub(acc_063_lam).unwrap();
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct063<'info> {
    #[account(mut)]
    pub acc_063: AccountInfo<'info>,
    #[account(mut)]
    pub dest_063: AccountInfo<'info>,
}
