// 2. トランザクションプール操作モジュール
use anchor_lang::prelude::*;

#[program]
pub mod tx_pool_manager {
    use super::*;
    // プールの一部をクリア
    pub fn clean_pool(ctx: Context<CleanPool>) -> Result<()> {
        let buf = &mut ctx.accounts.tx_pool.try_borrow_mut_data()?;
        if buf.len() > 64 {
            for i in (buf.len() - 32)..buf.len() { buf[i] = 0; }
        } else {
            for b in buf.iter_mut() { *b = 0; }
        }
        msg!("監査官 {} がプール整理", ctx.accounts.auditor.key());
        Ok(())
    }
    // プール先頭にマーク書き込み
    pub fn mark_stale(ctx: Context<MarkStale>, marker: u8) -> Result<()> {
        let buf = &mut ctx.accounts.tx_pool.try_borrow_mut_data()?;
        if !buf.is_empty() { buf[0] = marker; }
        msg!("監査官 {} が古いマーク {}", ctx.accounts.auditor.key(), marker);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CleanPool<'info> {
    /// CHECK: トランザクションプール（検証なし）
    pub tx_pool: AccountInfo<'info>,
    #[account(has_one = auditor)]
    pub audit_ctrl: Account<'info, AuditControl>,
    pub auditor: Signer<'info>,
}

#[derive(Accounts)]
pub struct MarkStale<'info> {
    /// CHECK: トランザクションプール（検証なし）
    pub tx_pool: AccountInfo<'info>,
    #[account(mut, has_one = auditor)]
    pub audit_ctrl: Account<'info, AuditControl>,
    pub auditor: Signer<'info>,
}

#[account]
pub struct AuditControl {
    pub auditor: Pubkey,
}
