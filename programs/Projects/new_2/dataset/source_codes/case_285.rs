// 2. トランザクション集計（ループでログ収集）
use anchor_lang::prelude::*;

#[program]
pub mod tx_aggregator {
    use super::*;
    pub fn aggregate(ctx: Context<Aggregate>, count: u8) -> Result<()> {
        let buf = &mut ctx.accounts.log_store.try_borrow_mut_data()?;
        // シンプルにバイト末尾をカウント分だけ記録
        for i in 0..count as usize {
            if buf.len() > i {
                buf[i] = buf[i].wrapping_add(1);
            }
        }
        msg!("監査官 {} が aggregate を実行", ctx.accounts.auditor.key());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Aggregate<'info> {
    /// CHECK: 生ログ保存用（検証なし）
    pub log_store: AccountInfo<'info>,
    #[account(has_one = auditor)]
    pub audit_ctrl: Account<'info, AuditControl>,
    pub auditor: Signer<'info>,
}

#[account]
pub struct AuditControl {
    pub auditor: Pubkey,
}
