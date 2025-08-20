use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_072 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_072(ctx: Context<CloseStruct072>) -> ProgramResult {
        let mut acc_072_lam = ctx.accounts.acc_072.to_account_info().lamports.borrow_mut();
        let mut dest_072_lam = ctx.accounts.dest_072.to_account_info().lamports.borrow_mut();
        // Snippet: checked_add with unwrap
        *dest_072_lam = dest_072_lam.checked_add(*acc_072_lam).unwrap();
        *acc_072_lam = 0;
    

        // Snippet: log before/after
        msg!("SRC={} DST={}", *acc_072_lam, *dest_072_lam);
        let new = *acc_072_lam + *dest_072_lam;
        *dest_072_lam = new;
    

        // Snippet: alias reference
        let dst_ref: &mut u64 = &mut *dest_072_lam;
        let src_val = *acc_072_lam;
        *dst_ref = dst_ref.checked_add(src_val).unwrap();
        *acc_072_lam = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct072<'info> {
    #[account(mut)]
    pub acc_072: AccountInfo<'info>,
    #[account(mut)]
    pub dest_072: AccountInfo<'info>,
}
