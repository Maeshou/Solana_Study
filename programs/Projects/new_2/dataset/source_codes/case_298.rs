// 5. メタフラグ管理モジュール
use anchor_lang::prelude::*;

#[program]
pub mod meta_flag_manager {
    use super::*;
    // フラグを反転またはクリア
    pub fn flip_or_clear(ctx: Context<FlipOrClear>, thresh: u8) -> Result<()> {
        let buf = &mut ctx.accounts.meta_flag.try_borrow_mut_data()?;
        if buf.len() < thresh as usize {
            for b in buf.iter_mut() { *b = !*b; }
        } else {
            for idx in 0..4.min(buf.len()) { buf[idx] = 0; }
        }
        msg!("監督者 {} がフラグ操作 (閾値={})", ctx.accounts.supervisor.key(), thresh);
        Ok(())
    }
    // フラグをデフォルト値にセット
    pub fn set_default(ctx: Context<SetDefault>) -> Result<()> {
        let buf = &mut ctx.accounts.meta_flag.try_borrow_mut_data()?;
        for b in buf.iter_mut() { *b = 0xFF; }
        msg!("監督者 {} がデフォルト設定", ctx.accounts.supervisor.key());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct FlipOrClear<'info> {
    /// CHECK: メタフラグ（検証なし）
    pub meta_flag: AccountInfo<'info>,
    #[account(has_one = supervisor)]
    pub sup_ctrl: Account<'info, SupervisorControl>,
    pub supervisor: Signer<'info>,
}

#[derive(Accounts)]
pub struct SetDefault<'info> {
    /// CHECK: メタフラグ（検証なし）
    pub meta_flag: AccountInfo<'info>,
    #[account(mut, has_one = supervisor)]
    pub sup_ctrl: Account<'info, SupervisorControl>,
    pub supervisor: Signer<'info>,
}

#[account]
pub struct SupervisorControl {
    pub supervisor: Pubkey,
}
