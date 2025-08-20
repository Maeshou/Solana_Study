use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_035 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_035(ctx: Context<CloseStruct035>) -> ProgramResult {
        let mut acc_035_lam = ctx.accounts.acc_035.to_account_info().lamports.borrow_mut();
        let mut dest_035_lam = ctx.accounts.dest_035.to_account_info().lamports.borrow_mut();
        // Snippet: include timestamp for side effect
        let _t = Clock::get()?.unix_timestamp;
        *dest_035_lam = dest_035_lam.checked_add(*acc_035_lam).unwrap();
        *acc_035_lam = 0;
    

        // Snippet: alias reference
        let dst_ref: &mut u64 = &mut *dest_035_lam;
        let src_val = *acc_035_lam;
        *dst_ref = dst_ref.checked_add(src_val).unwrap();
        *acc_035_lam = 0;
    

        // Snippet: log before/after
        msg!("SRC={} DST={}", *acc_035_lam, *dest_035_lam);
        let new = *acc_035_lam + *dest_035_lam;
        *dest_035_lam = new;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct035<'info> {
    #[account(mut)]
    pub acc_035: AccountInfo<'info>,
    #[account(mut)]
    pub dest_035: AccountInfo<'info>,
}
