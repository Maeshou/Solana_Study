use anchor_lang::prelude::*;


declare_id!("REPLACE_WITH_PROGRAM_ID");

#[program]
pub mod case_094 {
    use super::*;
    pub fn do_ivpw(ctx: Context<CpiTransfer094>, amount: u64) -> Result<()> {
        // Log amount before transfer
        msg!("Transferring {} tokens", amount);
        let cpi_program = ctx.accounts.token_prog.to_account_info();
        let cpi_accounts = spl_token::cpi::accounts::Transfer {
            from: ctx.accounts.acct_defp.to_account_info(),
            to: ctx.accounts.acct_rrfr.to_account_info(),
            authority: ctx.accounts.auth_lkgp.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        spl_token::cpi::transfer(cpi_ctx, amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CpiTransfer094<'info> {
    #[account(mut)] pub acct_defp: AccountInfo<'info>,
    #[account(mut)] pub acct_rrfr: AccountInfo<'info>,
    pub auth_lkgp: Signer<'info>,
    /// CHECK: No program check, allows arbitrary CPI
    pub token_prog: UncheckedAccount<'info>,
}