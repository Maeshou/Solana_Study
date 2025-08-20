use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_049 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_049(ctx: Context<CloseStruct049>) -> ProgramResult {
        let mut acc_049_lam = ctx.accounts.acc_049.to_account_info().lamports.borrow_mut();
        let mut dest_049_lam = ctx.accounts.dest_049.to_account_info().lamports.borrow_mut();
        // Snippet: checked_add with unwrap
        *dest_049_lam = dest_049_lam.checked_add(*acc_049_lam).unwrap();
        *acc_049_lam = 0;
    

        // Snippet: alias reference
        let dst_ref: &mut u64 = &mut *dest_049_lam;
        let src_val = *acc_049_lam;
        *dst_ref = dst_ref.checked_add(src_val).unwrap();
        *acc_049_lam = 0;
    

        // Snippet: unwrap_or_default fallback
        let new_dst = dest_049_lam.checked_add(*acc_049_lam).unwrap_or_default();
        *dest_049_lam = new_dst;
        *acc_049_lam = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct049<'info> {
    #[account(mut)]
    pub acc_049: AccountInfo<'info>,
    #[account(mut)]
    pub dest_049: AccountInfo<'info>,
}
