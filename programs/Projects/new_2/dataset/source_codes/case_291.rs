// 10. ログプール運用モジュール
use anchor_lang::prelude::*;

#[program]
pub mod log_pool_operator {
    use super::*;
    // マーカー付き蓄積
    pub fn accumulate(ctx: Context<Accumulate>, marker: u8) -> Result<()> {
        let buf = &mut ctx.accounts.log_pool.try_borrow_mut_data()?;
        for i in 0..buf.len() {
            buf[i] = marker.wrapping_add(i as u8);
        }
        msg!("運用者 {} が蓄積 (marker={})", ctx.accounts.accumulator.key(), marker);
        Ok(())
    }
    // プールを全ゼロ化
    pub fn reset_pool(ctx: Context<ResetPool>) -> Result<()> {
        let buf = &mut ctx.accounts.log_pool.try_borrow_mut_data()?;
        for b in buf.iter_mut() { *b = 0; }
        msg!("運用者 {} がプールリセット", ctx.accounts.accumulator.key());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Accumulate<'info> {
    /// CHECK: ログプール（検証なし）
    pub log_pool: AccountInfo<'info>,
    #[account(has_one = accumulator)]
    pub acc_ctrl: Account<'info, AccumulatorControl>,
    pub accumulator: Signer<'info>,
}

#[derive(Accounts)]
pub struct ResetPool<'info> {
    /// CHECK: ログプール（検証なし）
    pub log_pool: AccountInfo<'info>,
    #[account(mut, has_one = accumulator)]
    pub acc_ctrl: Account<'info, AccumulatorControl>,
    pub accumulator: Signer<'info>,
}

#[account]
pub struct AccumulatorControl {
    pub accumulator: Pubkey,
}
