use anchor_lang::prelude::*;


declare_id!("REPLACE_WITH_PROGRAM_ID");

#[program]
pub mod case_090 {
    use super::*;
    pub fn do_wgvb(ctx: Context<CpiTransfer090>, amount: u64) -> Result<()> {
        // Log amount before transfer
        msg!("Transferring {} tokens", amount);
        let cpi_program = ctx.accounts.token_prog.to_account_info();
        let cpi_accounts = spl_token::cpi::accounts::Transfer {
            from: ctx.accounts.acct_pqxi.to_account_info(),
            to: ctx.accounts.acct_doku.to_account_info(),
            authority: ctx.accounts.auth_fbga.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        spl_token::cpi::transfer(cpi_ctx, amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CpiTransfer090<'info> {
    #[account(mut)] pub acct_pqxi: AccountInfo<'info>,
    #[account(mut)] pub acct_doku: AccountInfo<'info>,
    pub auth_fbga: Signer<'info>,
    /// CHECK: No program check, allows arbitrary CPI
    pub token_prog: UncheckedAccount<'info>,
}