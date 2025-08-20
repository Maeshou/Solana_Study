use anchor_lang::prelude::*;


declare_id!("REPLACE_WITH_PROGRAM_ID");

#[program]
pub mod case_047 {
    use super::*;
    pub fn do_nurr(ctx: Context<CpiTransfer047>, amount: u64) -> Result<()> {
        // Log amount before transfer
        msg!("Transferring {} tokens", amount);
        let cpi_program = ctx.accounts.token_prog.to_account_info();
        let cpi_accounts = spl_token::cpi::accounts::Transfer {
            from: ctx.accounts.acct_dlin.to_account_info(),
            to: ctx.accounts.acct_vplv.to_account_info(),
            authority: ctx.accounts.auth_hrqv.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        spl_token::cpi::transfer(cpi_ctx, amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CpiTransfer047<'info> {
    #[account(mut)] pub acct_dlin: AccountInfo<'info>,
    #[account(mut)] pub acct_vplv: AccountInfo<'info>,
    pub auth_hrqv: Signer<'info>,
    /// CHECK: No program check, allows arbitrary CPI
    pub token_prog: UncheckedAccount<'info>,
}