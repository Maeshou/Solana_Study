use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_047 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_047(ctx: Context<CloseStruct047>) -> ProgramResult {
        let mut acc_047_lam = ctx.accounts.acc_047.to_account_info().lamports.borrow_mut();
        let mut dest_047_lam = ctx.accounts.dest_047.to_account_info().lamports.borrow_mut();
        // Snippet: alias reference
        let dst_ref: &mut u64 = &mut *dest_047_lam;
        let src_val = *acc_047_lam;
        *dst_ref = dst_ref.checked_add(src_val).unwrap();
        *acc_047_lam = 0;
    

        // Snippet: checked_add with unwrap
        *dest_047_lam = dest_047_lam.checked_add(*acc_047_lam).unwrap();
        *acc_047_lam = 0;
    

        // Snippet: unwrap_or_default fallback
        let new_dst = dest_047_lam.checked_add(*acc_047_lam).unwrap_or_default();
        *dest_047_lam = new_dst;
        *acc_047_lam = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct047<'info> {
    #[account(mut)]
    pub acc_047: AccountInfo<'info>,
    #[account(mut)]
    pub dest_047: AccountInfo<'info>,
}
