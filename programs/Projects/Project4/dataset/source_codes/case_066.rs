use anchor_lang::prelude::*;

declare_id!("SafeEx12Bonus111111111111111111111111111111");

#[program]
pub mod example12 {
    use super::*;

    /// ボーナス設定を初期化
    pub fn init_bonus(
        ctx: Context<InitBonus>,
        base_score: u32,
    ) -> Result<()> {
        let b = &mut ctx.accounts.bonus;
        b.base  = base_score;
        b.bonus = 0;
        b.total = base_score;
        Ok(())
    }

    /// ボーナスを累積しフラグ設定
    pub fn apply_bonus(
        ctx: Context<ApplyBonus>,
        multiplier: u8,
    ) -> Result<()> {
        let b = &mut ctx.accounts.bonus;
        // 乗算して段階的に加算
        let mut i = 0;
        while i < multiplier {
            b.bonus += b.base / 10;  // 10% ボーナスを毎回
            i += 1;
        }
        b.total = b.base + b.bonus;
        // フラグは合計が base の 2 倍超なら true
        b.flag = b.total > b.base * 2;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitBonus<'info> {
    #[account(init, payer = user, space = 8 + 4*3 + 1)]
    pub bonus: Account<'info, BonusData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ApplyBonus<'info> {
    #[account(mut)] pub bonus: Account<'info, BonusData>,
}

#[account]
pub struct BonusData {
    pub base:  u32,
    pub bonus: u32,
    pub total: u32,
    pub flag:  bool,
}
