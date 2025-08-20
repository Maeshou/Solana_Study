use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("TourA09MultiP4yN7Lm3R8tD6W4yZ1nC5bK2hU0X309");

#[program]
pub mod tournament_pool_multi_v1 {
    use super::*;

    pub fn init_pool(ctx: Context<InitPool>, prize_pool_input: u64) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        pool.operator = ctx.accounts.operator.key();
        pool.units = prize_pool_input;
        if pool.units < 30 { pool.units = 30; }
        pool.season = 1;
        pool.total_paid = 1;
        Ok(())
    }

    pub fn act_split(ctx: Context<ActSplit>, w1: u16, w2: u16, w3: u16) -> Result<()> {
        let pool = &mut ctx.accounts.pool;

        let total_weight = w1 as u64 + w2 as u64 + w3 as u64;
        let mut s1 = pool.units * w1 as u64 / total_weight;
        let mut s2 = pool.units * w2 as u64 / total_weight;
        let mut s3 = pool.units * w3 as u64 / total_weight;

        // 端数調整
        let now = s1 + s2 + s3;
        if now < pool.units { s1 = s1 + (pool.units - now); }

        token::transfer(ctx.accounts.pool_to_one(), s1)?;
        token::transfer(ctx.accounts.pool_to_two(), s2)?;
        token::transfer(ctx.accounts.pool_to_three(), s3)?;

        pool.total_paid = pool.total_paid + s1 + s2 + s3;
        pool.season = pool.season + 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitPool<'info> {
    #[account(init, payer = operator, space = 8 + 32 + 8 + 8 + 8)]
    pub pool: Account<'info, TourPool>,
    #[account(mut)]
    pub operator: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ActSplit<'info> {
    #[account(mut, has_one = operator)]
    pub pool: Account<'info, TourPool>,
    pub operator: Signer<'info>,

    #[account(mut)]
    pub pool_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub winner1_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub winner2_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub winner3_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}
impl<'info> ActSplit<'info> {
    pub fn pool_to_one(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let x = Transfer { from:self.pool_vault.to_account_info(), to:self.winner1_vault.to_account_info(), authority:self.operator.to_account_info() };
        CpiContext::new(self.token_program.to_account_info(), x)
    }
    pub fn pool_to_two(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let x = Transfer { from:self.pool_vault.to_account_info(), to:self.winner2_vault.to_account_info(), authority:self.operator.to_account_info() };
        CpiContext::new(self.token_program.to_account_info(), x)
    }
    pub fn pool_to_three(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let x = Transfer { from:self.pool_vault.to_account_info(), to:self.winner3_vault.to_account_info(), authority:self.operator.to_account_info() };
        CpiContext::new(self.token_program.to_account_info(), x)
    }
}
#[account]
pub struct TourPool {
    pub operator: Pubkey,
    pub units: u64,
    pub season: u64,
    pub total_paid: u64,
}
