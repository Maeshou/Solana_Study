use anchor_lang::prelude::*;
use anchor_spl::token::{self, MintTo, Token, TokenAccount, Mint};

declare_id!("Quiz10PointsA7vXk2Wm4Qy6Vt8Rb0Lc3Za5Hd7Q310");

#[program]
pub mod quiz_champ_points_v1 {
    use super::*;

    pub fn init_quiz(ctx: Context<InitQuiz>, daily_cap_input: u64, base_point_input: u64) -> Result<()> {
        let q = &mut ctx.accounts.quiz;
        q.host = ctx.accounts.host.key();
        q.daily_cap = daily_cap_input;
        if q.daily_cap < 20 { q.daily_cap = 20; }
        q.base_point = base_point_input;
        if q.base_point < 1 { q.base_point = 1; }
        q.issued_today = 0;
        q.streak_counter = 1;
        Ok(())
    }

    pub fn act_award(ctx: Context<ActAward>, answers_mask: u64, rounds_played: u8) -> Result<()> {
        let q = &mut ctx.accounts.quiz;

        // ビットスコア
        let mut score_units: u64 = 0;
        let mut bit_cursor: u8 = 0;
        while bit_cursor < 16 {
            let bit_flag = (answers_mask >> bit_cursor) & 1;
            if bit_flag == 1 { score_units = score_units + (bit_cursor as u64 + 1); }
            bit_cursor = bit_cursor + 1;
        }

        // ストリーク倍率
        let mut multiplier_percent: u64 = 100;
        let mut round_cursor: u8 = 0;
        while round_cursor < rounds_played {
            multiplier_percent = multiplier_percent + 3;
            round_cursor = round_cursor + 1;
        }

        let base_units: u64 = q.base_point + score_units / 3 + 1;
        let mut final_units: u64 = (base_units as u128 * multiplier_percent as u128 / 100u128) as u64;

        if q.streak_counter % 5 == 0 { final_units = final_units + 2; }
        if q.streak_counter % 8 == 0 { final_units = final_units + 3; }

        let projected = q.issued_today + final_units;
        if projected > q.daily_cap {
            let remainder = q.daily_cap - q.issued_today;
            if remainder > 0 { token::mint_to(ctx.accounts.mint_ctx(), remainder)?; }
            q.issued_today = q.daily_cap;
            q.streak_counter = 1;
            return Err(QuizErr::DailyCap.into());
        }

        token::mint_to(ctx.accounts.mint_ctx(), final_units)?;
        q.issued_today = projected;
        q.streak_counter = q.streak_counter + 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitQuiz<'info> {
    #[account(init, payer = host, space = 8 + 32 + 8 + 8 + 8 + 8)]
    pub quiz: Account<'info, QuizState>,
    #[account(mut)]
    pub host: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ActAward<'info> {
    #[account(mut, has_one = host)]
    pub quiz: Account<'info, QuizState>,
    pub host: Signer<'info>,

    pub point_mint: Account<'info, Mint>,
    #[account(mut)]
    pub player_point_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}
impl<'info> ActAward<'info> {
    pub fn mint_ctx(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let call = MintTo { mint: self.point_mint.to_account_info(), to: self.player_point_vault.to_account_info(), authority: self.host.to_account_info() };
        CpiContext::new(self.token_program.to_account_info(), call)
    }
}
#[account]
pub struct QuizState {
    pub host: Pubkey,
    pub daily_cap: u64,
    pub base_point: u64,
    pub issued_today: u64,
    pub streak_counter: u64,
}
#[error_code]
pub enum QuizErr { #[msg("daily cap reached")] DailyCap }
