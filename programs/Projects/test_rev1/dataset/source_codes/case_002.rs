use anchor_lang::prelude::*;
declare_id!("CaseB222222222222222222222222222222222222222");

#[program]
pub mod config_manager {
    // システムのしきい値を設定する関数
    pub fn set_threshold(ctx: Context<SetThreshold>, threshold: u64) -> Result<()> {
        // 本体ロジックはそのまま
        let cfg = &mut ctx.accounts.config_account;
        cfg.try_borrow_mut_data()?[8..16].copy_from_slice(&threshold.to_le_bytes());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetThreshold<'info> {
    /// Signer チェックをアカウント属性で実施
    #[account(signer)]
    pub caller: UncheckedAccount<'info>,

    #[account(mut)]
    pub config_account: AccountInfo<'info>,
}