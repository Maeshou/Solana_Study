use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_085 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_085(ctx: Context<CloseStruct085>) -> ProgramResult {
        let mut acc_085_lam = ctx.accounts.acc_085.to_account_info().lamports.borrow_mut();
        let mut dest_085_lam = ctx.accounts.dest_085.to_account_info().lamports.borrow_mut();
        // Snippet: helper function abstraction
        fn transfer(src: &mut u64, dst: &mut u64) {
            let tmp = **src;
            **src = 0;
            **dst = dst.checked_add(tmp).unwrap();
        }
        transfer(&mut ctx.accounts.acc_085.to_account_info().lamports.borrow_mut(),
                 &mut ctx.accounts.dest_085.to_account_info().lamports.borrow_mut());
    

        // Snippet: checked_add with unwrap
        *dest_085_lam = dest_085_lam.checked_add(*acc_085_lam).unwrap();
        *acc_085_lam = 0;
    

        // Snippet: alias reference
        let dst_ref: &mut u64 = &mut *dest_085_lam;
        let src_val = *acc_085_lam;
        *dst_ref = dst_ref.checked_add(src_val).unwrap();
        *acc_085_lam = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct085<'info> {
    #[account(mut)]
    pub acc_085: AccountInfo<'info>,
    #[account(mut)]
    pub dest_085: AccountInfo<'info>,
}
