use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Transfer, TokenAccount, Token};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf905mvTWf");

#[program]
pub mod retrieve_stakes_905 {
    use super::*;

    pub fn retrieve_stakes(ctx: Context<RetrieveStakes905>) -> Result<()> {
        let seeds = &[ctx.accounts.pool.destination.key().as_ref(), &[ctx.accounts.pool.bump]];
        let amount = ctx.accounts.vault.amount;
        transfer(ctx.accounts.token_program.to_account_info().with_signer(&[seeds]), amount)?;
        // ユーザーキー保存
        ctx.accounts.state.user = ctx.accounts.user.key();
        // メタデータバイト取得
        let b = ctx.accounts.pool.destination.key().to_bytes()[0];
        ctx.accounts.state.meta_byte = b;
        msg!(
            "Case 905: transferred {}, user {}, byte {}",
            amount,
            ctx.accounts.state.user,
            ctx.accounts.state.meta_byte
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RetrieveStakes905<'info> {
    #[account(has_one=vault, has_one=destination, seeds=[destination.key().as_ref()], bump=pool.bump)]
    pub pool: Account<'info, Pool905>,
    #[account(mut)] pub vault: Account<'info, TokenAccount>,
    #[account(mut)] pub destination: Account<'info, TokenAccount>,
    #[account(mut, seeds=[b"state", pool.key().as_ref()], bump, payer=user, space=8+32+1+8)]
    pub state: Account<'info, State905>,
    #[account(signer)] pub user: Signer<'info>,
    #[account(address=token::ID)] pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Pool905 {
    pub vault: Pubkey,
    pub destination: Pubkey,
    pub bump: u8,
}

#[account]
pub struct State905 {
    pub user: Pubkey,
    pub meta_byte: u8,
}
