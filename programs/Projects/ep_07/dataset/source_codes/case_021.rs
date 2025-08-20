use anchor_lang::prelude::*;


declare_id!("REPLACE_WITH_PROGRAM_ID");

#[program]
pub mod case_021 {
    use super::*;
    pub fn do_enzs(ctx: Context<CpiTransfer021>, amount: u64) -> Result<()> {
        // Log amount before transfer
        msg!("Transferring {} tokens", amount);
        let cpi_program = ctx.accounts.token_prog.to_account_info();
        let cpi_accounts = spl_token::cpi::accounts::Transfer {
            from: ctx.accounts.acct_lzrh.to_account_info(),
            to: ctx.accounts.acct_rwei.to_account_info(),
            authority: ctx.accounts.auth_oexz.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        spl_token::cpi::transfer(cpi_ctx, amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CpiTransfer021<'info> {
    #[account(mut)] pub acct_lzrh: AccountInfo<'info>,
    #[account(mut)] pub acct_rwei: AccountInfo<'info>,
    pub auth_oexz: Signer<'info>,
    /// CHECK: No program check, allows arbitrary CPI
    pub token_prog: UncheckedAccount<'info>,
}