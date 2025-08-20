use anchor_lang::prelude::*;
declare_id!("CaseB222222222222222222222222222222222222222");

#[program]
pub mod config_manager {
    // システムのしきい値を設定する関数
    pub fn set_threshold(ctx: Context<SetThreshold>, threshold: u64) -> Result<()> {
        // caller（Signer）チェックなし
        let cfg = &mut ctx.accounts.config_account;
        // ownerチェックなしで直接値を上書き
        cfg.try_borrow_mut_data()?[8..16].copy_from_slice(&threshold.to_le_bytes());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetThreshold<'info> {
    /// CHECK: 認証なしでだれでも呼び出し可能
    pub caller: UncheckedAccount<'info>,
    /// CHECK: プログラムIDとの照合を行わない
    #[account(mut)]
    pub config_account: AccountInfo<'info>,
}
