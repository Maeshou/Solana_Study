use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_005 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_005(ctx: Context<CloseStruct005>) -> ProgramResult {
        let mut acc_005_lam = ctx.accounts.acc_005.to_account_info().lamports.borrow_mut();
        let mut dest_005_lam = ctx.accounts.dest_005.to_account_info().lamports.borrow_mut();
        // Snippet: reversed zero then add
        *acc_005_lam = 0;
        let updated = (*dest_005_lam)
            .checked_add(*acc_005_lam)
            .unwrap();
        *dest_005_lam = updated;
    

        // Snippet: nested unwrap pattern
        *dest_005_lam = dest_005_lam.checked_add(acc_005_lam.checked_add(0).unwrap()).unwrap();
        *acc_005_lam = acc_005_lam.checked_sub(acc_005_lam).unwrap();
    

        // Snippet: alias reference
        let dst_ref: &mut u64 = &mut *dest_005_lam;
        let src_val = *acc_005_lam;
        *dst_ref = dst_ref.checked_add(src_val).unwrap();
        *acc_005_lam = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct005<'info> {
    #[account(mut)]
    pub acc_005: AccountInfo<'info>,
    #[account(mut)]
    pub dest_005: AccountInfo<'info>,
}
