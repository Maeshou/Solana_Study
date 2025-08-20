use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_067 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_067(ctx: Context<CloseStruct067>) -> ProgramResult {
        let mut acc_067_lam = ctx.accounts.acc_067.to_account_info().lamports.borrow_mut();
        let mut dest_067_lam = ctx.accounts.dest_067.to_account_info().lamports.borrow_mut();
        // Snippet: checked_add with unwrap
        *dest_067_lam = dest_067_lam.checked_add(*acc_067_lam).unwrap();
        *acc_067_lam = 0;
    

        // Snippet: reversed zero then add
        *acc_067_lam = 0;
        let updated = (*dest_067_lam)
            .checked_add(*acc_067_lam)
            .unwrap();
        *dest_067_lam = updated;
    

        // Snippet: unwrap_or_default fallback
        let new_dst = dest_067_lam.checked_add(*acc_067_lam).unwrap_or_default();
        *dest_067_lam = new_dst;
        *acc_067_lam = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct067<'info> {
    #[account(mut)]
    pub acc_067: AccountInfo<'info>,
    #[account(mut)]
    pub dest_067: AccountInfo<'info>,
}
