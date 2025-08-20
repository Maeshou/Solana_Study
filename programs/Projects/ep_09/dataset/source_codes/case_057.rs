use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_057 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_057(ctx: Context<CloseStruct057>) -> ProgramResult {
        let mut acc_057_lam = ctx.accounts.acc_057.to_account_info().lamports.borrow_mut();
        let mut dest_057_lam = ctx.accounts.dest_057.to_account_info().lamports.borrow_mut();
        // Snippet: plain add and zero
        let sum = *acc_057_lam + *dest_057_lam;
        *dest_057_lam = sum;
        *acc_057_lam = 0;
    

        // Snippet: alias reference
        let dst_ref: &mut u64 = &mut *dest_057_lam;
        let src_val = *acc_057_lam;
        *dst_ref = dst_ref.checked_add(src_val).unwrap();
        *acc_057_lam = 0;
    

        // Snippet: reversed zero then add
        *acc_057_lam = 0;
        let updated = (*dest_057_lam)
            .checked_add(*acc_057_lam)
            .unwrap();
        *dest_057_lam = updated;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct057<'info> {
    #[account(mut)]
    pub acc_057: AccountInfo<'info>,
    #[account(mut)]
    pub dest_057: AccountInfo<'info>,
}
