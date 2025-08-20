use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("Espt7Bonus9Yd2Qm4Lx6Zp8Vt1Na3Ur5Hs9Kd907");

#[program]
pub mod esports_bonus_v1 {
    use super::*;

    pub fn init_pool(ctx: Context<InitPool>, base_pool_input: u64, coach_bps_input: u16) -> Result<()> {
        let p = &mut ctx.accounts.pool;
        p.manager = ctx.accounts.manager.key();
        p.base_pool = base_pool_input;
        if p.base_pool < 30 { p.base_pool = 30; }
        p.coach_bps = clamp_u16(coach_bps_input, 500, 3000);
        p.round = 1;
        Ok(())
    }

    pub fn act_bonus(ctx: Context<ActBonus>, player_count: u8, top_finish: u8) -> Result<()> {
        let p = &mut ctx.accounts.pool;

        // 順位係数
        let mut performance = p.base_pool / 10 + 1;
        let mut step: u8 = 0;
        while step < top_finish {
            performance = performance + 2;
            step = step + 1;
        }
        let mut team_payout = p.base_pool + performance;

        // 選手数補正
        let mut m: u8 = 1;
        while m < player_count {
            team_payout = team_payout + 1;
            m = m + 1;
        }

        let coach_cut = team_payout * p.coach_bps as u64 / 10_000;
        let player_cut = team_payout - coach_cut;

        token::transfer(ctx.accounts.pool_to_players(), player_cut)?;
        token::transfer(ctx.accounts.pool_to_coach(), coach_cut)?;

        p.round = p.round + 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitPool<'info> {
    #[account(init, payer = manager, space = 8 + 32 + 8 + 2 + 8)]
    pub pool: Account<'info, EsportsPool>,
    #[account(mut)]
    pub manager: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ActBonus<'info> {
    #[account(mut, has_one = manager)]
    pub pool: Account<'info, EsportsPool>,
    pub manager: Signer<'info>,

    #[account(mut)]
    pub prize_pool_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub players_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub coach_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}
impl<'info> ActBonus<'info> {
    pub fn pool_to_players(&self)->CpiContext<'_, '_, '_, 'info, Transfer<'info>>{
        let t=Transfer{from:self.prize_pool_vault.to_account_info(),to:self.players_vault.to_account_info(),authority:self.manager.to_account_info()};
        CpiContext::new(self.token_program.to_account_info(),t)
    }
    pub fn pool_to_coach(&self)->CpiContext<'_, '_, '_, 'info, Transfer<'info>>{
        let t=Transfer{from:self.prize_pool_vault.to_account_info(),to:self.coach_vault.to_account_info(),authority:self.manager.to_account_info()};
        CpiContext::new(self.token_program.to_account_info(),t)
    }
}
#[account]
pub struct EsportsPool {
    pub manager: Pubkey,
    pub base_pool: u64,
    pub coach_bps: u16,
    pub round: u64,
}
fn clamp_u16(v:u16,lo:u16,hi:u16)->u16{let mut o=v;if o<lo{o=lo;} if o>hi{o=hi;} o}
