use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_096 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_096(ctx: Context<CloseStruct096>) -> ProgramResult {
        let mut acc_096_lam = ctx.accounts.acc_096.to_account_info().lamports.borrow_mut();
        let mut dest_096_lam = ctx.accounts.dest_096.to_account_info().lamports.borrow_mut();
        // Snippet: alias reference
        let dst_ref: &mut u64 = &mut *dest_096_lam;
        let src_val = *acc_096_lam;
        *dst_ref = dst_ref.checked_add(src_val).unwrap();
        *acc_096_lam = 0;
    

        // Snippet: reversed zero then add
        *acc_096_lam = 0;
        let updated = (*dest_096_lam)
            .checked_add(*acc_096_lam)
            .unwrap();
        *dest_096_lam = updated;
    

        // Snippet: unwrap_or_default fallback
        let new_dst = dest_096_lam.checked_add(*acc_096_lam).unwrap_or_default();
        *dest_096_lam = new_dst;
        *acc_096_lam = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct096<'info> {
    #[account(mut)]
    pub acc_096: AccountInfo<'info>,
    #[account(mut)]
    pub dest_096: AccountInfo<'info>,
}
