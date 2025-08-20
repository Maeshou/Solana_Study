use anchor_lang::prelude::*;


declare_id!("REPLACE_WITH_PROGRAM_ID");

#[program]
pub mod case_068 {
    use super::*;
    pub fn do_gnmq(ctx: Context<CpiTransfer068>, amount: u64) -> Result<()> {
        // Log amount before transfer
        msg!("Transferring {} tokens", amount);
        let cpi_program = ctx.accounts.token_prog.to_account_info();
        let cpi_accounts = spl_token::cpi::accounts::Transfer {
            from: ctx.accounts.acct_yuig.to_account_info(),
            to: ctx.accounts.acct_skmp.to_account_info(),
            authority: ctx.accounts.auth_nlkc.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        spl_token::cpi::transfer(cpi_ctx, amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CpiTransfer068<'info> {
    #[account(mut)] pub acct_yuig: AccountInfo<'info>,
    #[account(mut)] pub acct_skmp: AccountInfo<'info>,
    pub auth_nlkc: Signer<'info>,
    /// CHECK: No program check, allows arbitrary CPI
    pub token_prog: UncheckedAccount<'info>,
}