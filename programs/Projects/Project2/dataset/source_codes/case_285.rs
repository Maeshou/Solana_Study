use anchor_lang::prelude::*;

declare_id!("SeasonPass44444444444444444444444444444444");

#[program]
pub mod season_pass {
    use super::*;

    /// 経験値獲得
    pub fn gain_xp(ctx: Context<GainXp>, xp: u64) -> Result<()> {
        let p = &mut ctx.accounts.pass;
        p.xp = p.xp.saturating_add(xp);
        // レベル閾値を超えたらレベルアップ
        while p.level < p.thresholds.len() as u8 &&
              p.xp >= p.thresholds[p.level as usize] {
            p.xp -= p.thresholds[p.level as usize];
            p.level = p.level.saturating_add(1);
            p.bonuses[p.level as usize] = true;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct GainXp<'info> {
    #[account(mut)]
    pub pass: Account<'info, SeasonPassData>,
    pub user: Signer<'info>,
}

#[account]
pub struct SeasonPassData {
    pub xp: u64,
    pub level: u8,
    pub thresholds: Vec<u64>,
    pub bonuses: Vec<bool>,
}
