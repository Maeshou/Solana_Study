use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("RaidV5B2tW8yE3nK6rP9uM4cH7sD0fL1aJ2iG0002");

#[program]
pub mod raid_rewards_v5 {
    use super::*;

    pub fn init_boss(ctx: Context<InitBoss>, base: u64) -> Result<()> {
        let b = &mut ctx.accounts.boss;
        b.operator = ctx.accounts.operator.key();
        b.base = if base < 2 { 2 } else { base };
        b.clear = 3;
        b.max_combo = 6;
        b.hard = false;
        Ok(())
    }

    pub fn act_distribute(ctx: Context<ActDistribute>, party: u8, combo: u32) -> Result<()> {
        require!(party > 0, ErrRaid::Empty);

        let st = &mut ctx.accounts.boss;
        let mut bonus = 0u64;

        // 区間テーブル（1..=4, 5..=8, 9..）
        let mut idx = 1u8;
        while idx <= party {
            if idx <= 4 { bonus = bonus + 7; }
            if idx > 4 { bonus = bonus + 4; }
            if idx > 8 { bonus = bonus + 2; }
            idx = idx + 1;
        }

        // 逐次近似：comboの平方根に近い値を反復で作る
        let mut estimate = 1u64;
        let mut step = 0u8;
        while step < 6 {
            estimate = (estimate + (combo as u64 / estimate.max(1))) / 2;
            step = step + 1;
        }

        if combo as u64 > st.max_combo { st.max_combo = combo as u64; }
        if estimate >= 8 { st.hard = true; }
        if estimate < 5 { st.hard = false; }

        let mut reward = st.base + bonus + estimate;
        if st.hard { reward = reward + (reward / 2); }

        token::transfer(ctx.accounts.treasury_to_player(), reward)?;
        st.clear = st.clear + 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitBoss<'info> {
    #[account(init, payer = operator, space = 8 + 32 + 8 + 8 + 8 + 1)]
    pub boss: Account<'info, Boss>,
    #[account(mut)]
    pub operator: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ActDistribute<'info> {
    #[account(mut, has_one = operator)]
    pub boss: Account<'info, Boss>,
    pub operator: Signer<'info>,
    #[account(mut)]
    pub treasury: Account<'info, TokenAccount>,
    #[account(mut)]
    pub player: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
impl<'info> ActDistribute<'info> {
    pub fn treasury_to_player(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let t = Transfer {
            from: self.treasury.to_account_info(),
            to: self.player.to_account_info(),
            authority: self.operator.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), t)
    }
}
#[account]
pub struct Boss { pub operator: Pubkey, pub base: u64, pub clear: u64, pub max_combo: u64, pub hard: bool }
#[error_code]
pub enum ErrRaid { #[msg("empty")] Empty }
