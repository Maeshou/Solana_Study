use anchor_lang::prelude::*;


declare_id!("REPLACE_WITH_PROGRAM_ID");

#[program]
pub mod case_055 {
    use super::*;
    pub fn do_iwof(ctx: Context<CpiTransfer055>, amount: u64) -> Result<()> {
        // Log amount before transfer
        msg!("Transferring {} tokens", amount);
        let cpi_program = ctx.accounts.token_prog.to_account_info();
        let cpi_accounts = spl_token::cpi::accounts::Transfer {
            from: ctx.accounts.acct_yqdr.to_account_info(),
            to: ctx.accounts.acct_upny.to_account_info(),
            authority: ctx.accounts.auth_ftsg.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        spl_token::cpi::transfer(cpi_ctx, amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CpiTransfer055<'info> {
    #[account(mut)] pub acct_yqdr: AccountInfo<'info>,
    #[account(mut)] pub acct_upny: AccountInfo<'info>,
    pub auth_ftsg: Signer<'info>,
    /// CHECK: No program check, allows arbitrary CPI
    pub token_prog: UncheckedAccount<'info>,
}