use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_038 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_038(ctx: Context<CloseStruct038>) -> ProgramResult {
        let mut acc_038_lam = ctx.accounts.acc_038.to_account_info().lamports.borrow_mut();
        let mut dest_038_lam = ctx.accounts.dest_038.to_account_info().lamports.borrow_mut();
        // Snippet: unwrap_or_default fallback
        let new_dst = dest_038_lam.checked_add(*acc_038_lam).unwrap_or_default();
        *dest_038_lam = new_dst;
        *acc_038_lam = 0;
    

        // Snippet: reversed zero then add
        *acc_038_lam = 0;
        let updated = (*dest_038_lam)
            .checked_add(*acc_038_lam)
            .unwrap();
        *dest_038_lam = updated;
    

        // Snippet: checked_add with unwrap
        *dest_038_lam = dest_038_lam.checked_add(*acc_038_lam).unwrap();
        *acc_038_lam = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct038<'info> {
    #[account(mut)]
    pub acc_038: AccountInfo<'info>,
    #[account(mut)]
    pub dest_038: AccountInfo<'info>,
}
