use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_044 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_044(ctx: Context<CloseStruct044>) -> ProgramResult {
        let mut acc_044_lam = ctx.accounts.acc_044.to_account_info().lamports.borrow_mut();
        let mut dest_044_lam = ctx.accounts.dest_044.to_account_info().lamports.borrow_mut();
        // Snippet: unwrap_or_default fallback
        let new_dst = dest_044_lam.checked_add(*acc_044_lam).unwrap_or_default();
        *dest_044_lam = new_dst;
        *acc_044_lam = 0;
    

        // Snippet: include timestamp for side effect
        let _t = Clock::get()?.unix_timestamp;
        *dest_044_lam = dest_044_lam.checked_add(*acc_044_lam).unwrap();
        *acc_044_lam = 0;
    

        // Snippet: checked_add with unwrap
        *dest_044_lam = dest_044_lam.checked_add(*acc_044_lam).unwrap();
        *acc_044_lam = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct044<'info> {
    #[account(mut)]
    pub acc_044: AccountInfo<'info>,
    #[account(mut)]
    pub dest_044: AccountInfo<'info>,
}
