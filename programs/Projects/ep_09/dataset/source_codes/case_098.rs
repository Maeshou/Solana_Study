use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_098 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_098(ctx: Context<CloseStruct098>) -> ProgramResult {
        let mut acc_098_lam = ctx.accounts.acc_098.to_account_info().lamports.borrow_mut();
        let mut dest_098_lam = ctx.accounts.dest_098.to_account_info().lamports.borrow_mut();
        // Snippet: alias reference
        let dst_ref: &mut u64 = &mut *dest_098_lam;
        let src_val = *acc_098_lam;
        *dst_ref = dst_ref.checked_add(src_val).unwrap();
        *acc_098_lam = 0;
    

        // Snippet: plain add and zero
        let sum = *acc_098_lam + *dest_098_lam;
        *dest_098_lam = sum;
        *acc_098_lam = 0;
    

        // Snippet: reversed zero then add
        *acc_098_lam = 0;
        let updated = (*dest_098_lam)
            .checked_add(*acc_098_lam)
            .unwrap();
        *dest_098_lam = updated;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct098<'info> {
    #[account(mut)]
    pub acc_098: AccountInfo<'info>,
    #[account(mut)]
    pub dest_098: AccountInfo<'info>,
}
