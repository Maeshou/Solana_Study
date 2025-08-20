use anchor_lang::prelude::*;

declare_id!("LgStk01111111111111111111111111111111111");

#[program]
pub mod login_streak {
    use super::*;

    pub fn record_login(ctx: Context<RecLogin>, current_slot: u64) -> Result<()> {
        let s = &mut ctx.accounts.streak;
        // 前回ログインから経過日数を算出（slot 単位）
        let days_passed = (current_slot - s.last_slot) / s.slots_per_day;
        // 1日以内の再ログインなら連続継続、そうでなければリセット
        if days_passed == 1 {
            s.streak = s.streak.saturating_add(1);
            s.last_slot = current_slot;
            s.bonus_points += 10;
        } else {
            s.streak = 1;
            s.last_slot = current_slot;
            s.bonus_points = 5;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RecLogin<'info> {
    #[account(mut)]
    pub streak: Account<'info, StreakData>,
}

#[account]
pub struct StreakData {
    pub streak: u64,
    pub last_slot: u64,
    pub slots_per_day: u64,
    pub bonus_points: u64,
}
