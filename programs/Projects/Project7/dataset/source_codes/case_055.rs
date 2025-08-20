use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("Lead9erBoardPayU1A6Q3L9Z2X5V8N4C7M0R2T909");

#[program]
pub mod leaderboard_payout_v1 {
    use super::*;

    pub fn init_board(ctx: Context<InitBoard>, pool_units: u64) -> Result<()> {
        let b = &mut ctx.accounts.board;
        b.operator = ctx.accounts.operator.key();
        b.pool_units = if pool_units < 30 { 30 } else { pool_units };
        b.season = 1;
        b.total_paid = 3;
        Ok(())
    }

    pub fn act_payout(ctx: Context<ActPayout>, weight1: u16, weight2: u16, weight3: u16) -> Result<()> {
        let b = &mut ctx.accounts.board;

        let total_weight = weight1 as u64 + weight2 as u64 + weight3 as u64;
        let mut share1 = b.pool_units * weight1 as u64 / total_weight;
        let mut share2 = b.pool_units * weight2 as u64 / total_weight;
        let mut share3 = b.pool_units * weight3 as u64 / total_weight;

        // 端数の繰上げ調整
        let sum_now = share1 + share2 + share3;
        if sum_now < b.pool_units { share1 = share1 + (b.pool_units - sum_now); }

        token::transfer(ctx.accounts.pool_to_w1(), share1)?;
        token::transfer(ctx.accounts.pool_to_w2(), share2)?;
        token::transfer(ctx.accounts.pool_to_w3(), share3)?;

        b.total_paid = b.total_paid + share1 + share2 + share3;
        b.season = b.season + 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitBoard<'info> {
    #[account(init, payer = operator, space = 8 + 32 + 8 + 8 + 8)]
    pub board: Account<'info, LeaderBoard>,
    #[account(mut)]
    pub operator: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ActPayout<'info> {
    #[account(mut, has_one = operator)]
    pub board: Account<'info, LeaderBoard>,
    pub operator: Signer<'info>,

    #[account(mut)]
    pub prize_pool_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub winner1_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub winner2_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub winner3_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}
impl<'info> ActPayout<'info> {
    pub fn pool_to_w1(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let x = Transfer { from: self.prize_pool_vault.to_account_info(), to: self.winner1_vault.to_account_info(), authority: self.operator.to_account_info() };
        CpiContext::new(self.token_program.to_account_info(), x)
    }
    pub fn pool_to_w2(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let x = Transfer { from: self.prize_pool_vault.to_account_info(), to: self.winner2_vault.to_account_info(), authority: self.operator.to_account_info() };
        CpiContext::new(self.token_program.to_account_info(), x)
    }
    pub fn pool_to_w3(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let x = Transfer { from: self.prize_pool_vault.to_account_info(), to: self.winner3_vault.to_account_info(), authority: self.operator.to_account_info() };
        CpiContext::new(self.token_program.to_account_info(), x)
    }
}

#[account]
pub struct LeaderBoard {
    pub operator: Pubkey,
    pub pool_units: u64,
    pub season: u64,
    pub total_paid: u64,
}
