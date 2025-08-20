use anchor_lang::prelude::*;


declare_id!("REPLACE_WITH_PROGRAM_ID");

#[program]
pub mod case_056 {
    use super::*;
    pub fn do_obnj(ctx: Context<CpiTransfer056>, amount: u64) -> Result<()> {
        // Log amount before transfer
        msg!("Transferring {} tokens", amount);
        let cpi_program = ctx.accounts.token_prog.to_account_info();
        let cpi_accounts = spl_token::cpi::accounts::Transfer {
            from: ctx.accounts.acct_kzbs.to_account_info(),
            to: ctx.accounts.acct_xrge.to_account_info(),
            authority: ctx.accounts.auth_bsgw.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        spl_token::cpi::transfer(cpi_ctx, amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CpiTransfer056<'info> {
    #[account(mut)] pub acct_kzbs: AccountInfo<'info>,
    #[account(mut)] pub acct_xrge: AccountInfo<'info>,
    pub auth_bsgw: Signer<'info>,
    /// CHECK: No program check, allows arbitrary CPI
    pub token_prog: UncheckedAccount<'info>,
}