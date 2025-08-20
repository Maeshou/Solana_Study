use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_spl::token::{approve, Approve, burn, Burn, TokenAccount, Token};
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf652mvTWf");

#[program]
pub mod dispatch_operation_652 {
    use super::*;

    pub fn dispatch_operation(ctx: Context<DispatchOperation652>, lamports: u64, app: u64, brn: u64) -> Result<()> {
        let sys_tx = system_program::Transfer {
            from: ctx.accounts.payer.to_account_info(),
            to: ctx.accounts.receiver.to_account_info(),
        };
        system_program::transfer(CpiContext::new(ctx.accounts.sys_prog.to_account_info(), sys_tx), lamports)?;
        let ap = Approve {
            to: ctx.accounts.acc.to_account_info(),
            delegate: ctx.accounts.delegate.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        };
        approve(CpiContext::new(ctx.accounts.token_prog.to_account_info(), ap), app)?;
        let br = Burn {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.acc.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        };
        burn(CpiContext::new(ctx.accounts.token_prog.to_account_info(), br), brn)?;
        let md = &mut ctx.accounts.meta;
        md.count_lp = md.count_lp.checked_add(lamports).unwrap();
        md.count_ap = md.count_ap.checked_add(app).unwrap();
        md.count_br = md.count_br.checked_add(brn).unwrap();
        msg!(
            "Case 652: lp {} ap {} br {}; totals: {} {} {}",
            lamports, app, brn, md.count_lp, md.count_ap, md.count_br
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct DispatchOperation652<'info> {
    #[account(address=system_program::ID)] pub sys_prog: Program<'info, System>,
    #[account(mut)] pub payer: Signer<'info>,
    #[account(mut)] pub receiver: SystemAccount<'info>,
    #[account(address=token::ID)] pub token_prog: Program<'info, Token>,
    #[account(mut)] pub mint: Account<'info, anchor_spl::token::Mint>,
    #[account(mut)] pub acc: Account<'info, TokenAccount>,
    pub delegate: UncheckedAccount<'info>,
    #[account(mut)] pub meta: Account<'info, Meta652>,
}

#[account]
pub struct Meta652 {
    pub count_lp: u64,
    pub count_ap: u64,
    pub count_br: u64,
}
