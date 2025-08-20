use anchor_lang::prelude::*;

declare_id!("SafeEx33ScoreMult11111111111111111111111111");

#[program]
pub mod example33 {
    use super::*;

    pub fn init_multiplier(
        ctx: Context<InitMultiplier>,
        base: u32,
        mult: u8,
    ) -> Result<()> {
        let s = &mut ctx.accounts.score;
        s.base_score       = base;
        s.multiplier       = mult;
        s.adjusted_score   = base;

        // 単純乗算
        s.adjusted_score = s.base_score.saturating_mul(s.multiplier as u32);
        Ok(())
    }

    pub fn apply_multiplier(
        ctx: Context<ApplyMultiplier>,
        extra: u8,
    ) -> Result<()> {
        let s = &mut ctx.accounts.score;
        let new_mult = s.multiplier.saturating_add(extra);
        s.multiplier = if new_mult > 10 { 10 } else { new_mult };

        // 再計算とボーナス
        s.adjusted_score = s.base_score.saturating_mul(s.multiplier as u32);
        if s.multiplier > 5 {
            s.adjusted_score = s.adjusted_score.saturating_add(50);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitMultiplier<'info> {
    #[account(init, payer = user, space = 8 + 4 + 1 + 4)]
    pub score: Account<'info, ScoreMultiplierData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ApplyMultiplier<'info> {
    #[account(mut)] pub score: Account<'info, ScoreMultiplierData>,
}

#[account]
pub struct ScoreMultiplierData {
    pub base_score:     u32,
    pub multiplier:     u8,
    pub adjusted_score: u32,
}
