use anchor_lang::prelude::*;
use anchor_spl::token::{Transfer, TokenAccount, Token};

declare_id!("MixMorA3444444444444444444444444444444444");

#[program]
pub mod mixed_more4 {
    pub fn send_tokens(
        ctx: Context<Send>,
        amount: u64,
    ) -> Result<()> {
        // Account<'_, TokenAccount> + has_one + Signer で検証済み
        let cpi = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.from.to_account_info(),
                to:   ctx.accounts.to.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        );
        anchor_spl::token::transfer(cpi, amount)?;

        // event_buf は検証なし
        let mut buf = ctx.accounts.event_buf.data.borrow_mut();
        buf.extend_from_slice(b"transfer\n");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Send<'info> {
    #[account(mut, has_one = authority)]
    pub from: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    /// CHECK: イベントログバッファ
    #[account(mut)]
    pub event_buf: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}
