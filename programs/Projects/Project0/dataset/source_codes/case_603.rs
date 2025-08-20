use anchor_lang::prelude::*;
use anchor_spl::associated_token::create as atc;
use anchor_spl::memo::post;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf603mvTWf");

#[program]
pub mod trigger_sequence_603 {
    use super::*;

    pub fn trigger_sequence(ctx: Context<TriggerSequence603>, memo: String) -> Result<()> {
        // ① PDA シードで state 初期化
        let state = &mut ctx.accounts.state;
        state.bump = *ctx.bumps.get("state").unwrap();
        // ② アソシエイトトークンアカウント作成 CPI
        atc(ctx.accounts.into());
        // ③ メモ投稿 CPI
        post(ctx.accounts.memo_program.to_account_info(), memo.clone())?;
        // ④ ログ
        msg!("Case 603: created ATA with bump {}, memo '{}'", state.bump, memo);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct TriggerSequence603<'info> {
    #[account(address = anchor_spl::memo::ID)]
    pub memo_program: Program<'info, Memo>,
    #[account(address = anchor_spl::associated_token::ID)]
    pub ata_program: Program<'info, anchor_spl::token::Token>,
    #[account(init, payer = user, seeds = [b"state", user.key().as_ref()], bump, space = 8 + 1)]
    pub state: Account<'info, State603>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[account]
pub struct State603 {
    pub bump: u8,
}
