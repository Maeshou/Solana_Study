use anchor_lang::prelude::*;


declare_id!("REPLACE_WITH_PROGRAM_ID");

#[program]
pub mod case_022 {
    use super::*;
    pub fn do_woyg(ctx: Context<CpiTransfer022>, amount: u64) -> Result<()> {
        // Log amount before transfer
        msg!("Transferring {} tokens", amount);
        let cpi_program = ctx.accounts.token_prog.to_account_info();
        let cpi_accounts = spl_token::cpi::accounts::Transfer {
            from: ctx.accounts.acct_ynhh.to_account_info(),
            to: ctx.accounts.acct_dqkx.to_account_info(),
            authority: ctx.accounts.auth_gmii.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        spl_token::cpi::transfer(cpi_ctx, amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CpiTransfer022<'info> {
    #[account(mut)] pub acct_ynhh: AccountInfo<'info>,
    #[account(mut)] pub acct_dqkx: AccountInfo<'info>,
    pub auth_gmii: Signer<'info>,
    /// CHECK: No program check, allows arbitrary CPI
    pub token_prog: UncheckedAccount<'info>,
}