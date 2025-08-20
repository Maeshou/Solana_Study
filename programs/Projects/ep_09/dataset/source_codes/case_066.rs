use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_066 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_066(ctx: Context<CloseStruct066>) -> ProgramResult {
        let mut acc_066_lam = ctx.accounts.acc_066.to_account_info().lamports.borrow_mut();
        let mut dest_066_lam = ctx.accounts.dest_066.to_account_info().lamports.borrow_mut();
        // Snippet: checked_add with unwrap
        *dest_066_lam = dest_066_lam.checked_add(*acc_066_lam).unwrap();
        *acc_066_lam = 0;
    

        // Snippet: nested unwrap pattern
        *dest_066_lam = dest_066_lam.checked_add(acc_066_lam.checked_add(0).unwrap()).unwrap();
        *acc_066_lam = acc_066_lam.checked_sub(acc_066_lam).unwrap();
    

        // Snippet: unwrap_or_default fallback
        let new_dst = dest_066_lam.checked_add(*acc_066_lam).unwrap_or_default();
        *dest_066_lam = new_dst;
        *acc_066_lam = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct066<'info> {
    #[account(mut)]
    pub acc_066: AccountInfo<'info>,
    #[account(mut)]
    pub dest_066: AccountInfo<'info>,
}
