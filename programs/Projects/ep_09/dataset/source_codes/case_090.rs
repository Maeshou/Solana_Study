use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_090 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_090(ctx: Context<CloseStruct090>) -> ProgramResult {
        let mut acc_090_lam = ctx.accounts.acc_090.to_account_info().lamports.borrow_mut();
        let mut dest_090_lam = ctx.accounts.dest_090.to_account_info().lamports.borrow_mut();
        // Snippet: helper function abstraction
        fn transfer(src: &mut u64, dst: &mut u64) {
            let tmp = **src;
            **src = 0;
            **dst = dst.checked_add(tmp).unwrap();
        }
        transfer(&mut ctx.accounts.acc_090.to_account_info().lamports.borrow_mut(),
                 &mut ctx.accounts.dest_090.to_account_info().lamports.borrow_mut());
    

        // Snippet: checked_add with unwrap
        *dest_090_lam = dest_090_lam.checked_add(*acc_090_lam).unwrap();
        *acc_090_lam = 0;
    

        // Snippet: clone lamports slices
        let lam1 = ctx.accounts.acc_090.to_account_info().lamports().clone();
        let lam2 = ctx.accounts.dest_090.to_account_info().lamports().clone();
        let tot = lam2.checked_add(*lam1.borrow()).unwrap();
        *ctx.accounts.dest_090.to_account_info().lamports.borrow_mut() = tot;
        *ctx.accounts.acc_090.to_account_info().lamports.borrow_mut() = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct090<'info> {
    #[account(mut)]
    pub acc_090: AccountInfo<'info>,
    #[account(mut)]
    pub dest_090: AccountInfo<'info>,
}
